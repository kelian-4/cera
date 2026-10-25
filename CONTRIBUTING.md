**Français** | [English](docs/en/CONTRIBUTING.en.md)

# Contribuer à Cera

## Pour commencer

- Récupérez le dépôt et passez sur `dev` : `git clone https://github.com/kelian-4/cera && git switch dev`.
- Outils : Rust (la CI utilise **1.98.1**) avec `rustfmt` et `clippy`. Pour la suite de tests complète.
- Le code est organisé comme décrit dans la section « Structure du dépôt » du [README](README.md).

## Traductions

Les textes de l'interface sont dans `src/i18n/lang/` : un fichier JSON par langue (`fr.json`, `en.json`, `es.json`), intégré au programme à la compilation. Chaque fichier est un dictionnaire plat : la même clé dans toutes les langues, et le texte traduit.

```json
{
  "help.save": "save",
  "status.line_cut": "Line cut"
}
```

Le préfixe de la clé indique où le texte apparaît : `help` (bandeau d'aide), `doc` (documentation `Ctrl-D`), `status` (barre de statut), `cli` (ligne de commande et `--help`), `ui` et `search`.

La langue est choisie par la clé `language` de `config.json`, sinon par `LC_ALL` puis `LANG` (`fr_FR.UTF-8` donne `fr`), sinon l'anglais. Un texte absent d'une langue s'affiche en anglais, et le nom de la clé s'il manque aussi dans l'anglais.

### Corriger une traduction

Modifiez la valeur dans le fichier de la langue. Aucun code à toucher.

### Ajouter un texte

1. Ajoutez la clé dans **tous** les fichiers de langue. Le fichier anglais est la référence : `cargo test i18n` échoue si une langue n'a pas une clé présente dans `en.json`.
2. Pour insérer une valeur dans le texte, écrivez `%s` (un seul par texte). Gardez-le dans chaque traduction.
3. Dans le code, utilisez `i18n.t("ma.cle")`, ou `i18n.t1("ma.cle", valeur)` quand le texte contient un `%s`.

Les dictionnaires actuels n'utilisent pas d'accents : gardez cette convention pour le français, l'espagnol et les langues proches. Pour une langue qui ne s'écrit pas ainsi, ouvrez d'abord une discussion.

### Ajouter une langue

1. Copiez `src/i18n/lang/en.json` vers `src/i18n/lang/<code>.json` (code à deux lettres : `de`, `it`…), puis traduisez les valeurs. Ne changez ni les clés ni les `%s`.
2. Déclarez la langue dans `src/i18n/mod.rs`, dans la liste `EMBEDDED_LANGUAGES` :
   ```rust
   ("de", include_str!("lang/de.json")),
   ```
3. Lancez `cargo test i18n` : il vérifie que le fichier est valide et qu'il contient toutes les clés. Pour l'essayer, mettez `"language": "de"` dans `config.json` ou lancez Cera avec `LANG=de_DE.UTF-8`.
4. Ajoutez la langue à la liste de la section « Traductions » du [README](README.md).

### Ce qui n'est pas encore traduit

- les messages d'erreur d'arguments invalides de la ligne de commande (`describe_cli_error` dans `src/main.rs`) : français uniquement ;
- les noms d'options et leurs exemples dans `cera --help` (`--afficher`, `[dossier]`, `fichier...`) ;
- le nom, la description et le champ `help` des plugins : un seul texte par manifeste.

Les traduire est bienvenu.

## Signaler un problème

Voir la section « Signaler un problème » du [README](README.md). Pour un plantage, ajoutez la sortie du programme et, si possible, le contenu du fichier édité.

## Guide du développeur

| Pour… | Lisez |
|---|---|
| écrire un plugin ou changer le protocole | [PLUGINS.md](PLUGINS.md) |
| ajouter un langage de coloration | « Ajouter un langage » ci-dessous |
| les textes affichés | `src/i18n/lang/{fr,en,es}.json` |

## Branches et pull requests

- `dev` : branche de travail. Les pull requests visent `dev`.
- Une pull request fait une seule chose. N'y mélangez pas de changements cosmétiques sur des fichiers sans rapport.
- Ne changez pas un comportement, un raccourci ou une option sans que ce soit demandé. En cas de doute, ouvrez d'abord une discussion.

## Tests

Avant chaque commit vos modifications.(Oui je dois vraiment le préciser)

- Les tests unitaires sont dans les modules (`#[cfg(test)]`), les tests d'intégration dans `tests/`.
- Un test qui doit échouer sans votre correction se vérifie ainsi : retirez la correction, constatez l'échec, remettez-la.
- `CERA_REQUIRE_PLUGIN_LANGS=1 cargo test` rend obligatoires tous les outils externes (`python3`, `node`, `cc`, `c++`, `rustc`). Sans cette variable, le test d'un langage dont l'outil manque est ignoré. La CI l'active.

### Tester un comportement interactif

Une touche, un clic, un collage, un rendu ou le cycle de vie d'un plugin ne sont **jamais terminés sur la seule base d'une relecture du code** : ils se vérifient dans un terminal. `tests/escape_key.rs` et `tests/paste.rs` donnent le modèle : un script Python lance Cera dans un `pty`, envoie des octets, puis lit l'écran ou le fichier sauvegardé.

- Lancez ces scripts en arrière-plan, avec une alarme de sécurité (`signal.alarm`) dans le script, et tuez toujours le processus Cera à la fin.
- Lisez la **dernière frame** de l'écran (`texte.split('\x1b[H')[-1]`), pas tout le flux.
- Les plugins sont lus depuis `~/.config/cera/plugins` : après avoir modifié un plugin, relancez `./plugins/install.sh` (ou `HOME=<dossier de test> bash plugins/install.sh`) avant de tester.

## Règles de code

- **Pas de commentaires explicatifs** dans le code : des noms de variables et de fonctions clairs les remplacent. Seuls les shebangs sont conservés. Les fichiers `.md` ne sont pas concernés.
- **Chaque texte affiché à l'utilisateur** passe par `src/i18n/lang/{fr,en,es}.json` (clés plates). Ajoutez la clé dans les trois fichiers. Les dictionnaires actuels n'utilisent pas d'accents : gardez cette convention.

## Ajouter un langage de coloration

Chaque langage a son fichier `src/syntax/<id>_syntax.rs`.

1. Créez le fichier sur le modèle d'un langage existant (extensions, mots-clés, types, constantes, commentaires, chaînes multi-lignes, drapeaux…).
2. Déclarez-le dans `src/syntax/mod.rs` (`pub mod` et ordre de `build_database`). Une extension ne doit appartenir qu'à un seul langage ; un motif sans point (`Makefile`) est cherché dans le nom entier.
3. Ajoutez son entrée dans `plugins/pretty-cat/languages.json` : c'est la copie lue par `pretty-cat`.
4. Ajoutez un échantillon dans `src/syntax/language_samples.rs` : une ou plusieurs lignes et les couleurs attendues de quelques mots.
5. Lancez `cargo test syntax` : les tests de parité (`src/syntax/parity_tests.rs`) vérifient que `languages.json` est identique à la base Rust et que `pretty-cat` colore comme l'éditeur.

Un mot-clé suivi de `|` reçoit la seconde couleur de mot-clé (fonctions et constantes intégrées).

Le moteur existe en **deux versions qui doivent rester identiques** : `src/syntax/engine.rs` (éditeur) et `plugins/pretty-cat/main.py` (plugin). Toute modification de l'un se reporte dans l'autre ; les tests de parité échouent sinon.

## Ajouter un plugin 

Vous êtes libre de créer vos plugins, les soumettre au projet ou non. Mais les plugins du projet sont soumis à la même si licence qui lui et reste le propriété du projet

Choisissez un raccourci libre. Les lettres `Ctrl` déjà utilisées sont `a c d e f h k l n p q r s v x y z` ; restent `b g o t u w`. `i`, `j` et `m` sont inutilisables (Tab, saut de ligne, Entrée). Ne réutilisez jamais un raccourci existant.
