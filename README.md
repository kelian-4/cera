**Français** | [English](docs/en/README.en.md)

# Cera

Cera est un éditeur de texte pour le terminal, écrit en Rust.

<p align="center">
  <img src="docs/cera-demo.gif" alt="Démonstration de Cera" width="100%" />
</p>

<details open>
<summary><h2>Qu'est-ce que Cera ?</h2></summary>

Cera est un éditeur cli simple à prendre en main. Il est pensé comme une alternative plus moderne et plus intuitive aux éditeurs classiques avec un système de plugins pouvant être écrits en C++, C, Rust, JS, Python permettant d'étendre ou de modifier l'éditeur à son bon vouloir sans modifier le code source.

</details>

<details open>
<summary><h2>Images</h2></summary>

File explorer

<img width="1341" height="706" alt="2026-10-12_08:13:40" src="https://github.com/user-attachments/assets/efbd73e4-edda-4c00-bc57-7cb2b39db7ac" />


Pretty-cat 

<img width="1321" height="673" alt="2026-10-12_09:53:26" src="https://github.com/user-attachments/assets/26534a12-8e1c-4112-af36-203db33e5006" />

et plus

La liste complète des fonctionnalités et des raccourcis est dans l'éditeur : `Ctrl-D`, ou `cera --help` pour les options.

</details>

<details open>
<summary><h2>Installation</h2></summary>

### Avec Nix

```
nix run github:kelian-4/cera       # essayer sans installer
nix build                          # construire dans ./result
nix develop                        # shell de développement
```

Dans une configuration NixOS qui utilise les flakes :

```nix
inputs.cera = {
  url = "github:kelian-4/cera";
  inputs.nixpkgs.follows = "nixpkgs";
};

# dans la liste des modules :
{ environment.systemPackages = [ inputs.cera.packages.${system}.default ]; }
```
Au premier lancement, Cera copie les plugins fournis dans `~/.config/cera/plugins` et les active : voir [PLUGINS.md](PLUGINS.md).

### Depuis les sources

Il faut uniquement avoir Rust installé :

```
cargo build --release
./target/release/cera
./plugins/install.sh      # copie les plugins dans ~/.config/cera/plugins
```

Les plugins fournis demandent `python3`.

</details>

<details open>
<summary><h2>Utilisation</h2></summary>

### Sans plugin

```
cera [fichier]
```

### Avec plugin

```
cera [options] [fichier ...]
```

| Option | Effet |
|---|---|
| `-e`, `--explorer [dossier]` | Ouvre l'explorateur de fichiers sur un dossier |
| `-a`, `--afficher fichier...` | Affiche des fichiers en lecture seule, colorés |
| `-f`, `--find nom` | Cherche des fichiers par leur nom |
| `-g`, `--grep motif [chemin...]` | Cherche un texte dans des fichiers et dossiers |
| `-d`, `--diff fichier1 fichier2...` | Compare chaque fichier au premier |
| `-h`, `--help` | Affiche l'aide et la documentation des plugins actifs |
| `-v`, `--version` | Affiche la version |

Les options `-a`, `-f`, `-g` et `-d` sont fournies par des plugins : elles ne fonctionnent que si le plugin est activé.

</details>

<details open>
<summary><h2>Documentation</h2></summary>

- **Dans l'éditeur** : `Ctrl-D` ouvre la documentation complète (raccourcis, plugins actifs) ; `Ctrl-H` affiche ou masque le bandeau d'aide.
- **En ligne de commande** : `cera --help`.
- **Plugins** : [PLUGINS.md](PLUGINS.md) explique comment en utiliser et en écrire.

</details>

<details open>
<summary><h2>Configuration</h2></summary>

Au premier lancement, Cera crée `~/.config/cera/config.json`. Ce fichier contient toutes les clés disponibles avec leurs valeurs par défaut : largeur des tabulations, couleurs, langue de l'interface, plugins activés, souris, numéros de ligne. Modifiez-le directement ; une clé absente garde sa valeur par défaut.

</details>

<details open>
<summary><h2>Traductions</h2></summary>

L'interface de Cera existe en français, en anglais et en espagnol. La langue est choisie dans cet ordre :

1. la clé `language` de `config.json` (`"fr"`, `"en"` ou `"es"`) ;
2. la langue du système (variables `LC_ALL`, puis `LANG`) ;
3. l'anglais.

Un texte qui manque dans une traduction s'affiche en anglais. La description des plugins (`Ctrl-D`, `cera --help`) est dans la langue de leur manifeste : elle n'est pas traduite.

Pour corriger une traduction ou ajouter une langue, voir « Traductions » dans [CONTRIBUTING.md](CONTRIBUTING.md).

</details>

<details open>
<summary><h2>Structure du dépôt</h2></summary>

```
src/                  code source de l'éditeur
├─ editor/            boucle principale, buffers, annulation, presse-papier
├─ terminal/          mode brut, lecture des touches, souris, collage encadré
├─ syntax/            moteur de coloration et un fichier par langage
├─ plugin/            protocole, sous-processus, découverte des plugins
├─ view/              rendu de l'écran, aide intégrée
├─ io/                configuration, lecture/écriture de fichiers, JSON
└─ i18n/              traductions (fr, en, es)
plugins/              plugins fournis (voir PLUGINS.md)
tests/                tests d'intégration et plugins d'exemple en 5 langages
flake.nix             paquet et environnement de développement Nix
```

</details>

<details open>
<summary><h2>Contribuer</h2></summary>

Voir [CONTRIBUTING.md](CONTRIBUTING.md).

</details>

<details open>
<summary><h2>Signaler un problème</h2></summary>

Cherchez d'abord si le problème est déjà signalé dans les [issues](https://github.com/kelian-4/cera/issues). Sinon, ouvrez-en une en indiquant :

- votre système et votre terminal ;
- la version de Cera (le commit ou la version du paquet) ;
- le contenu utile de votre `config.json` et la liste des plugins actifs (`cera --help`) ;
- les étapes pour reproduire le problème.

</details>

<details open>
<summary><h2>Licence</h2></summary>

Cera est distribué sous licence GNU GPL version 3 ou ultérieure : voir [LICENSE](LICENSE).

</details>
