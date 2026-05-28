# Changelog

Toutes les évolutions notables du projet sont documentées ici, une section par
version (cf. `.features/vX.Y-*.md` pour les specs détaillées). Format inspiré de
[Keep a Changelog](https://keepachangelog.com/fr/1.1.0/).

## [v0.3-pop] — 2026-05-28

Phase 1 — retrait du top de la stack.

### Ajouté
- Opcode `POP` (`0x50`) : retire et jette le top de la stack, `pc += 1`.
- Helper `pop1()` qui factorise `StackUnderflow` (préfigure `pop2()`/`pop3()`).
- Tests natifs (`pop.rs`) + extension de l'oracle revm (`pop_matches_revm`, `pop_underflow_matches_revm`).

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (19 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : `PUSH1 x, POP` → stack identique ; `POP` sur stack vide → underflow des deux côtés (`InstructionResult::StackUnderflow` / `EvmError::StackUnderflow`).

## [v0.2-push1] — 2026-05-28

Phase 1 — premier opcode avec opérande immédiat.

### Ajouté
- Opcode `PUSH1` (`0x60`) : lit l'octet suivant, le pousse (zero-extended en `U256`), `pc += 2`.
- `Opcode::advance()` : nombre d'octets dont avance le PC (`2` pour `PUSH1`, `1` par défaut) — anticipe `PUSH2`..`PUSH32` (`advance = 1 + n`).
- `step` unifie l'avance via `pc += opcode.advance()` (évite la boucle infinie « `pc += 1` puis re-lecture »).
- Limite de stack à 1024 éléments → `StackOverflow`.
- Re-export public `evm::U256` (alias `ruint`).
- Tests natifs (`push1.rs`) + extension de l'oracle revm (`push1_matches_revm`, 6 programmes).

### Choix techniques
- **Immédiat manquant = `0`** (`[0x60]` → push `0`, puis STOP implicite), conforme à revm/geth, et non une erreur comme suggéré par la spec — seul comportement validable contre l'oracle revm.

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (14 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : 6 programmes `PUSH1` (valeur, zero-ext, immédiat manquant, double push, halt implicite) → mêmes résultat et stack que revm.

## [v0.1-stop] — 2026-05-28

Phase 1 — premier opcode et première validation contre `revm`.

### Ajouté
- Opcode `STOP` (`0x00`) : `Opcode::Stop` + décodage `TryFrom<u8>` (octet inconnu → `UnknownOpcode`) + `Opcode::as_byte()`.
- `interpreter::step` : décode l'opcode courant ; `STOP` → `halted = true`, `pc += 1`.
- `interpreter::run` : boucle `while !halted { step }`.
- Tests natifs (`stop.rs`) : halt + `pc=1` + stack vide, STOP multiples, code vide, opcode inconnu, `step` après halt.
- **Validation `revm`** (`revm_oracle.rs`) : exécution du même bytecode via l'interpréteur standalone de revm (`DummyHost`), comparaison du résultat d'instruction et de la stack avec notre interpréteur.

### Choix techniques
- **PC hors borne = `STOP` implicite** (comme revm/geth) plutôt que `InvalidPc` ; confirmé par l'oracle revm sur le bytecode vide.
- `Opcode` exhaustif (pas de `#[non_exhaustive]`) : le `match` du `step` force la gestion compile-time de chaque opcode ajouté.
- `revm` 40.0.3 ajouté en **dev-dependency** uniquement (n'impacte pas le build `no_std` de la lib).

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (8 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : `[0x00]` et `[]` → mêmes résultat et stack que revm.

## [v0.0-setup] — 2026-05-28

Phase 0 — mise en place du squelette. Aucun code métier (aucun opcode), donc
pas de validation contre `revm` à ce stade.

### Ajouté
- Workspace Cargo (`Cargo.toml`, resolver v2) avec le crate `crates/evm`.
- `EvmState` minimal : `stack: Vec<U256>`, `pc`, `halted`, `code` (+ constante
  `STACK_LIMIT = 1024`).
- `EvmError` : `StackUnderflow`, `StackOverflow`, `UnknownOpcode(u8)`, `Halted`,
  `InvalidPc` (avec `Display` et `std::error::Error` derrière la feature `std`).
- `Opcode` : enum placeholder (les opcodes réels arrivent en v0.1).
- `interpreter::step()` : squelette qui lit l'octet courant et renvoie
  `UnknownOpcode` (ou `InvalidPc` / `Halted` selon l'état).
- Tests d'intégration `it_compiles` et `empty_code_is_invalid_pc`.
- CI GitHub Actions : `cargo fmt --check`, `cargo clippy --all-targets -D warnings`,
  `cargo test`.

### Choix techniques
- **U256 = `ruint`** (et non `primitive-types`) : c'est la crate utilisée par
  `revm`, ce qui simplifiera la validation croisée et l'écosystème ZK.
- **Compatibilité `no_std`** anticipée dès maintenant (feature `std` activée par
  défaut, `alloc::vec::Vec`) pour préparer la phase 2 (SP1 / cible RISC-V).
- `.gitignore` corrigé (`!.github/`) pour que le workflow CI soit bien versionné.
- `Cargo.lock` versionné (projet applicatif → builds CI reproductibles).

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` : OK.
- Build `--no-default-features` (chemin `no_std`) : OK.
- Validation `revm` : sans objet (pas encore d'opcode).
