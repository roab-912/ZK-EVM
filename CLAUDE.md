# CLAUDE.md — Conventions de développement

## Environnement : tout passe par WSL Debian

Tout développement, build, test et installation de toolchain se fait dans **WSL Debian**, jamais en PowerShell natif Windows.

- Chemin du repo côté WSL : `/mnt/c/Users/Barbi/Documents/Code/GitHub/ZK-EVM`
- Forme des commandes :
  ```
  wsl -d Debian -- bash -lc "cd /mnt/c/Users/Barbi/Documents/Code/GitHub/ZK-EVM && <commande>"
  ```
- Toolchain disponible dans WSL Debian : `rustc`/`cargo` 1.95, `git` 2.47.
- `gh` 2.46.0 est installé dans WSL Debian → ouvrir les PR via `gh` (nécessite `gh auth login` une première fois).

## Workflow par feature (une version = une branche = une PR)

Les versions et leurs specs sont dans `.features/vX.Y-<nom>.md` (cf. `.features/ZK EVM Roadmap.md`).

Pour chaque feature :

1. **Branche** — partir de `main` à jour et créer une branche nommée comme le tag de la version :
   ```
   git checkout main && git pull
   git checkout -b v0.0-setup        # ex. pour v0.0
   ```
2. **Développement** — tout le travail se fait sur cette branche, dans WSL Debian.
3. **Critère de PR (bloquant)** — la PR n'est ouverte **que lorsque** :
   - la suite de tests passe et **vérifie complètement** le code développé, et
   - le résultat est **validé contre `revm`** sur les programmes de test concernés.
   Tant que ces deux conditions ne sont pas réunies, on ne crée pas de PR.
4. **PR** — ouvrir la PR de la branche `vX.Y-<nom>` vers `main` une fois les critères remplis.

Remote : `git@github.com:roab-912/ZK-EVM.git` (branche par défaut `main`).

## Validation contre revm

`revm` est l'oracle de référence : pour les opcodes/programmes implémentés, le résultat (stack, mémoire, return data, state root selon la phase) doit être identique à celui de `revm`. Une feature n'est « terminée » qu'avec cette validation verte.
