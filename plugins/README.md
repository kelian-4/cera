# Plugins Cera

Un plugin Cera est un executable independant, ecrit dans n'importe quel
langage, lance par Cera en sous-processus. La communication se fait par
des messages JSON, un par ligne, sur l'entree/sortie standard du plugin.

## Installer les plugins fournis

```
./plugins/install.sh
```

Copie chaque plugin de ce dossier vers `~/.config/cera/plugins/` (ou
`$XDG_CONFIG_HOME/cera/plugins/` si defini). Un plugin installe la n'est
actif que s'il figure dans `plugins.enabled` de `config.json` (les deux
plugins fournis le sont par defaut).

## Structure d'un plugin

```
mon-plugin/
  plugin.json   # manifeste
  main.py       # (ou tout autre executable/script)
```

`plugin.json` :

```json
{
  "name": "Mon Plugin",
  "description": "Ce que fait le plugin, en une phrase",
  "command": "python3",
  "args": ["main.py"],
  "shortcut": "^E"
}
```

- `command` : l'executable a lancer. Un nom simple (`python3`, `node`,
  `bash`) est cherche dans le `PATH` ; un chemin contenant `/` est resolu
  par rapport au dossier du plugin.
- `args` : arguments passes a `command`. Un argument qui ne commence pas
  par `/` est lui aussi resolu par rapport au dossier du plugin (c'est
  typiquement le chemin du script principal, comme `"main.py"`).
- `shortcut` : raccourci Ctrl suggere (ex: `"^E"` pour Ctrl-E), affiche a
  titre informatif ; c'est bien ce raccourci qui declenche le plugin dans
  Cera. `Ctrl-G` liste toujours tous les plugins actifs si vous ne vous
  souvenez plus de leurs raccourcis.

## Cycle de vie

1. Cera lance l'executable et lui envoie un message `init`.
2. Le plugin envoie un ou plusieurs messages `render` pour afficher une
   vue plein ecran a la place du buffer.
3. Cera transmet chaque touche pressee au plugin via des messages `key`,
   jusqu'a ce que le plugin reponde `open_file` ou `close`, ou que
   l'utilisateur appuie sur Echap (qui ferme toujours la vue cote Cera,
   meme si le plugin ne repond plus).
4. Cera termine le sous-processus (`shutdown`, puis un signal si besoin)
   avant de revenir a l'edition normale.

## Messages Cera -> plugin

Un message par ligne, sur l'entree standard du plugin.

- `{"type": "init", "screen_rows": N, "screen_cols": N, "args": ["..."]}`
  Envoye une seule fois au demarrage. `args` contient un seul element :
  le fichier actuellement ouvert dans Cera, ou le repertoire courant si
  aucun fichier n'est ouvert.
- `{"type": "key", "key": "Down"}`
  Une touche pressee par l'utilisateur. Valeurs possibles : `Up`, `Down`,
  `Left`, `Right`, `PageUp`, `PageDown`, `Home`, `End`, `Delete`,
  `Backspace`, `Enter`, `Tab`, `Escape`, ou un caractere unique (`"a"`,
  `"3"`, `"é"`...) pour une touche de texte normale.
- `{"type": "resize", "screen_rows": N, "screen_cols": N}`
  Le terminal a change de taille.
- `{"type": "shutdown"}`
  Cera demande au plugin de se terminer proprement.

## Messages plugin -> Cera

Un message par ligne, sur la sortie standard du plugin. Toute ligne qui
n'est pas du JSON reconnu est ignoree silencieusement par Cera (pratique
pour du debug ponctuel avec un `print`, mais preferez `sys.stderr` qui
n'est pas lu par Cera).

- `{"type": "render", "lines": [...]}`
  Remplace tout l'ecran par ces lignes. Chaque ligne est
  `{"segments": [...]}`, chaque segment
  `{"text": "...", "color": "#RRGGBB", "bold": true}` (`color` et `bold`
  sont optionnels ; sans `color`, le segment herite de la couleur de
  texte normale du theme de Cera).
- `{"type": "open_file", "path": "/chemin/vers/fichier"}`
  Demande a Cera d'ouvrir ce fichier en edition normale (ferme la vue du
  plugin).
- `{"type": "status", "message": "...", "kind": "info"}`
  Affiche un message dans la barre de statut de Cera. `kind` vaut `info`
  ou `error`.
- `{"type": "close"}`
  Ferme la vue du plugin et revient a l'edition normale (equivalent a ce
  que fait Echap cote utilisateur, mais a l'initiative du plugin).

## Plugins fournis

- **file-explorer** (`^E`) : arborescence de fichiers navigable au
  clavier (Haut/Bas pour se deplacer, Entree pour ouvrir un fichier ou
  plier/deplier un dossier, Droite/Gauche pour deplier/replier).
- **pretty-cat** (`cera -a fichier...`) : affiche un fichier en lecture seule avec
  numeros de ligne et coloration syntaxique basique (Haut/Bas/PageUp/
  PageDown pour faire defiler).

- **finder** (`cera -f nom`) : cherche recursivement, depuis le repertoire
  courant, les fichiers dont le nom contient `nom` (sans tenir compte de
  la casse) et affiche leur chemin, un par ligne. Code de sortie 1 si
  rien n'est trouve. Sans raccourci, utilisable uniquement en ligne de
  commande.
- **diff** (`cera -d fichier1 fichier2 ...`) : compare chaque fichier au
  premier et affiche un diff unifie colore (`+` en vert, `-` en rouge).
  Code de sortie 0 si tout est identique, 1 s'il y a des differences,
  2 en cas d'erreur de lecture. Sans raccourci, utilisable uniquement en
  ligne de commande.

Tous sont ecrits en Python (stdlib seule, aucune dependance) a titre
d'exemple ; rien n'empeche d'ecrire un plugin dans un autre langage tant
qu'il respecte le protocole ci-dessus.
