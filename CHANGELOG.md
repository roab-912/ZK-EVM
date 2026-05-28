# Changelog

Toutes les évolutions notables du projet sont documentées ici, une section par
version (cf. `.features/vX.Y-*.md` pour les specs détaillées). Format inspiré de
[Keep a Changelog](https://keepachangelog.com/fr/1.1.0/).

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
