# Smart Road

```
 ______
|___  /                                ___    _
   / /    ___    _ __     ___         / _ \  / |
  / /    / _ \  | '_ \   / _ \       | | | | | |
 / /__  | (_) | | | | | |  __/       | |_| | | |
/_____|  \___/  |_| |_|  \___|        \___/  |_|
```

Smart Road est un projet de simulation d'un carrefour à six voies avec des véhicules arrivant aléatoirement de toutes les directions. Ce programme est développé en Rust et utilise SDL2 pour l'affichage.

## 🌟 Fonctionnalités

- Simulation dynamique d'un carrefour à six voies.
- Gestion aléatoire des arrivées de véhicules.
- Utilisation de Rust pour la performance et la sécurité.
- Affichage géré par SDL2.

## 💪 Prérequis

### Rust & SDL2

Avant d'exécuter ce projet, vous devez installer **Rust** et **SDL2**.

#### Installation sur Linux et macOS

```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Vérifier l'installation de Rust
rustc --version

# Installer SDL2 (macOS uniquement)
brew install sdl2_image sdl2_mixer sdl2_ttf sdl2_gfx
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
export C_INCLUDE_PATH="$C_INCLUDE_PATH:/opt/homebrew/include"
source ~/.zshrc
```

## 💪 Utilisation

Pour exécuter la simulation, utilisez la commande suivante :

```bash
cargo run
```

## Commandes en jeu :

Utilisez les flèches directionnelles pour faire apparaître des véhicules venant de chaque côté du carrefour.
Appuyez sur R pour faire apparaître des véhicules de manière aléatoire.
Appuyez sur W pour quitter la fenêtre de simulation.

## 👨‍💻 Auteurs

Ce projet a été réalisé par :

- [Pierre P.](https://zone01normandie.org/git/ppeuraud)
- [Alan L.](https://zone01normandie.org/git/alebrume)
- [Clement R.](https://zone01normandie.org/git/cringuet)
- [Lucas P.](https://zone01normandie.org/git/lporte)
