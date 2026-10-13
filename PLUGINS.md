**Français** | [English](en/PLUGINS.en.md)

# Plugins

Un plugin est un **exécutable indépendant**, dans n'importe quel langage, lancé par Cera en sous-processus. Cera et le plugin échangent des messages JSON, sur l'entrée et la sortie standard du plugin.

Ce document explique comment en utiliser un, puis comment en écrire un.

## Démarrage rapide

Créez le dossier `~/.config/cera/plugins/bonjour/` avec deux fichiers.

`plugin.json` :

```json
{
  "name": "Bonjour",
  "description": "Exemple minimal",
  "help": "Ctrl-W : affiche un message, q pour fermer",
  "command": "python3",
  "args": ["main.py"],
  "shortcut": "^W"
}
```

`main.py` :

```python
#!/usr/bin/env python3
import json
import sys


def send(message):
    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


for raw in sys.stdin:
    try:
        message = json.loads(raw)
    except json.JSONDecodeError:
        continue
    kind = message.get("type")
    if kind == "init":
        send({"type": "render", "lines": [{"segments": [{"text": "Bonjour, q pour fermer", "bold": True}]}]})
    elif kind == "key" and message["key"] == "q":
        send({"type": "close"})
    elif kind == "shutdown":
        break
```

Ajoutez `"bonjour"` à `plugins.enabled` dans `config.json`, relancez Cera et tapez `Ctrl-W`. C'est tout.

## Trois façons d'utiliser un plugin

| Mode | Déclaration | Comportement |
|---|---|---|
| Plein écran | `"mode": "fullscreen"` (valeur par défaut) | La vue du plugin remplace tout l'écran, l'éditeur est suspendu |
| Panneau | `"mode": "panel"` | Panneau persistant à gauche, l'éditeur reste utilisable à droite |
| Ligne de commande | option fixe de Cera (`-a`, `-f`, `-g`, `-d`) | Cera lance le plugin avec `--cat`, `--find`, `--grep` ou `--diff` et lui passe les arguments |

Les options de ligne de commande sont **intégrées à Cera** : un plugin tiers ne peut pas en ajouter sans modifier le code de Cera. Pour les quatre plugins fournis, Cera exécute `<command> <args du manifeste> --cat|--find|--grep|--diff <arguments de l'utilisateur>`, laisse le plugin écrire directement sur la sortie standard, et sort avec le code de sortie du plugin.

## Installation et activation

```
./plugins/install.sh
```

copie chaque plugin du dépôt vers `~/.config/cera/plugins/` (ou `$XDG_CONFIG_HOME/cera/plugins/`). Cera cherche les plugins dans ce dossier, ou dans celui de `plugins.directory` (`config.json`).

Un plugin n'est actif que s'il est dans `plugins.enabled`. Si cette liste est absente, tous les plugins fournis sont actifs ; une liste explicite restreint les plugins actifs. Le `config.json` existant n'est jamais modifié par une mise à jour : ajoutez-y les nouveaux plugins à la main.


## Le manifeste `plugin.json`

| Champ | Rôle |
|---|---|
| `name` | Nom affiché (doc `Ctrl-D`, `--help`) |
| `description` | Une phrase |
| `help` | **Documentation minimale, obligatoire pour les plugins fournis** : une ligne courte (moins de 60 caractères idéalement) qui dit comment l'utiliser. Elle apparaît dans la catégorie « Plugins » de la doc `Ctrl-D` et dans `cera --help`. Si absente, `description` est utilisée. Un seul texte, sans traduction |
| `command` | L'exécutable. Un nom simple (`python3`, `node`) est cherché dans le `PATH` ; un chemin contenant `/` est résolu par rapport au dossier du plugin |
| `args` | Arguments passés à `command`. Un argument qui ne commence pas par `/` est résolu par rapport au dossier du plugin (typiquement `"main.py"`) |
| `shortcut` | Raccourci `Ctrl` qui lance le plugin, par exemple `"^E"`. Facultatif : sans lui, le plugin n'est utilisable que par une option de ligne de commande |
| `mode` | `"fullscreen"` (défaut) ou `"panel"` |

L'identifiant d'un plugin est le nom de son dossier : c'est celui qu'on met dans `plugins.enabled`.

**Raccourcis** : les lettres `Ctrl` déjà prises sont `a c d e f h k l n p q r s v x y z`. Restent libres `b g o t u w`. `i`, `j` et `m` sont inutilisables (Tab, saut de ligne, Entrée).

## Messages de Cera vers le plugin

- `{"type":"init","mode":"fullscreen","screen_rows":24,"screen_cols":80,"args":["/chemin"]}`
  Envoyé une seule fois au démarrage. En mode panneau, le message contient en plus `"panel_width"` (30 par défaut). `args` contient un seul élément : le fichier ouvert dans Cera, ou le dossier courant si aucun fichier n'est ouvert.
- `{"type":"key","key":"Down","focused":true}`
  Une touche pressée. `key` vaut `Up`, `Down`, `Left`, `Right`, `PageUp`, `PageDown`, `Home`, `End`, `Delete`, `Backspace`, `Enter`, `Tab`, `Escape`, ou un caractère unique (`"a"`, `"3"`, `"é"`…). Les combinaisons avec `Ctrl` ne sont pas transmises.
- `{"type":"click","row":3}` (mode panneau) : clic de souris dans le panneau. `row` est l'indice de la ligne cliquée **dans la liste des lignes que le plugin a envoyée** : 0 est la première ligne de son `render`. Ne soustrayez rien pour un éventuel en-tête : il n'y en a pas dans cette zone.
- `{"type":"resize","screen_rows":30,"screen_cols":100}` : le terminal a changé de taille.
- `{"type":"focus","focused":false}` (mode panneau) : le panneau prend ou perd le focus.
- `{"type":"shutdown"}` : Cera demande au plugin de se terminer proprement.

## Messages du plugin vers Cera

- `{"type":"render","lines":[...]}` : remplace l'affichage du plugin. Une ligne est `{"segments":[...]}` ; un segment est `{"text":"...","color":"#RRGGBB","bold":true}` (`color` et `bold` sont facultatifs ; sans `color`, le texte prend la couleur normale du thème). En mode panneau, le message peut contenir `"panel_width"` pour demander une autre largeur (bornée entre 10 colonnes et la largeur de l'écran moins 10).
- `{"type":"open_file","path":"/chemin/fichier"}` : demande à Cera d'ouvrir ce fichier en édition (ferme la vue plein écran ; retire le focus du panneau). Seul le chemin est transmis : il n'est pas possible d'indiquer une ligne.
- `{"type":"status","message":"...","kind":"info"}` : affiche un message dans la barre de statut. `kind` vaut `info` ou `error`.
- `{"type":"close"}` : ferme la vue du plugin (plein écran) ou le panneau.

Toute ligne de sortie qui n'est pas un JSON reconnu est ignorée silencieusement. Ce que le plugin écrit sur sa sortie d'erreur n'est pas lu par Cera : utilisez-la pour déboguer.

## Cycle de vie

### Plein écran

1. Cera lance le plugin et lui envoie `init`.
2. Le plugin envoie des messages `render`, qui remplacent tout l'écran.
3. Cera lui transmet chaque touche (`key`) et chaque changement de taille (`resize`).
4. La vue se termine quand le plugin envoie `open_file` ou `close`, ou quand l'utilisateur appuie sur `Échap` (qui ferme toujours la vue, même si le plugin ne répond plus).
5. Cera envoie `shutdown`, puis termine le processus après un court délai si besoin.

### Panneau

1. Le raccourci ouvre le panneau, qui prend le focus. Il ne se ferme pas tout seul.
2. Les touches ne sont transmises que lorsque le panneau a le focus. `Échap` retire le focus (le plugin reçoit `focus` avec `false`) et ne ferme pas le panneau.
3. Un clic dans le panneau lui donne le focus (`focus` avec `true`) puis envoie `click`. Un clic dans l'éditeur lui retire le focus.
4. Le raccourci redonne le focus à un panneau ouvert sans focus, et le **ferme** s'il a déjà le focus.
5. `open_file` ouvre le fichier et retire le focus, mais laisse le panneau ouvert. `close` ferme le panneau.
6. Si le processus se termine, le panneau se ferme et la barre de statut signale l'erreur.

Un seul panneau est ouvert à la fois : en ouvrir un autre ferme le premier. La largeur est de 30 colonnes par défaut.

## Conseils

- Videz (`flush`) la sortie après chaque message ; sinon Cera ne voit rien.
- Ignorez les messages dont vous ne connaissez pas le type (le protocole peut s'enrichir).
- Quittez quand vous recevez `shutdown` ou quand l'entrée se ferme.
- Évitez les bibliothèques tierces : les plugins fournis n'utilisent que la bibliothèque standard de Python.

## Langages testés

Le protocole est vérifié automatiquement (`tests/plugin_languages.rs`) avec un plugin d'exemple dans chaque langage ci-dessous, en mode plein écran et en mode panneau :

| Langage | Exemple | Lancé avec |
|---|---|---|
| Python | `tests/plugin_examples/hello.py` | `python3` |
| JavaScript | `tests/plugin_examples/hello.js` | `node` |
| C | `tests/plugin_examples/hello.c` | binaire compilé avec `cc` |
| C++ | `tests/plugin_examples/hello.cpp` | binaire compilé avec `c++` |
| Rust | `tests/plugin_examples/hello.rs` | binaire compilé avec `rustc` |

Chaque exemple gère `init`, `key` (y compris un caractère non ASCII), `resize`, `click`, `focus` et `shutdown`, et émet `render` (avec couleur et gras), `status`, `open_file` et `close`. Il écrit aussi une ligne qui n'est pas du JSON au démarrage, pour vérifier qu'elle est ignorée. Ce sont de bons points de départ.

## Tester un plugin

En isolation, sans lancer Cera :

```
printf '%s\n%s\n' \
  '{"type":"init","mode":"panel","screen_rows":10,"screen_cols":30,"panel_width":28,"args":["/tmp"]}' \
  '{"type":"shutdown"}' | python3 main.py
```

Chaque ligne de sortie doit être un JSON valide. Pour un comportement interactif, testez avec Cera dans un pseudo-terminal (voir [CONTRIBUTING.md](CONTRIBUTING.md)). Relancez `./plugins/install.sh` après chaque modification : Cera exécute la copie de `~/.config/cera/plugins`, pas celle du dépôt.

## Plugins fournis

| Plugin | Usage | Description |
|---|---|---|
| `file-explorer` | `Ctrl-E`, `cera -e [dossier]` | Panneau d'arborescence : Haut/Bas pour se déplacer, `Entrée` pour ouvrir un fichier ou plier un dossier, Droite/Gauche pour déplier/replier |
| `pretty-cat` | `cera -a fichier...` | Affiche des fichiers en lecture seule, avec numéros de ligne et coloration des 70 langages de l'éditeur. Ses définitions de langages sont dans `languages.json`, à côté de `main.py` |
| `finder` | `cera -f nom` | Cherche récursivement, depuis le dossier courant, les fichiers dont le nom contient `nom` (sans tenir compte de la casse) ; chemins relatifs, un par ligne ; code de sortie 1 si rien n'est trouvé |
| `grep` | `cera -g motif [chemin...]` | Cherche un texte littéral, sensible à la casse, dans des fichiers et dossiers (récursif) ; sortie `chemin:ligne:texte` ; fichiers binaires ignorés ; code de sortie 1 si rien n'est trouvé |
| `diff` | `cera -d f1 f2 ...` | Compare chaque fichier au premier, en diff unifié coloré ; code de sortie 0 (identiques), 1 (différences) ou 2 (erreur de lecture) |

## Limites actuelles

- Un plugin ne peut pas modifier le texte du fichier ouvert, ni changer le thème.
- `open_file` ne transmet qu'un chemin, sans numéro de ligne.
- Une seule vue plein écran et un seul panneau à la fois.
- Les options de ligne de commande sont fixes.
