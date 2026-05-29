# Changelog

Toutes les évolutions notables du projet sont documentées ici, une section par
version (cf. `.features/vX.Y-*.md` pour les specs détaillées). Format inspiré de
[Keep a Changelog](https://keepachangelog.com/fr/1.1.0/).

## [v0.9-sp1] — 2026-05-29

Phase 2 — branchement à SP1 : première preuve zkVM. **Aucun nouvel opcode**, que
de l'infrastructure. On prouve l'exécution de `programs/add.hex`
(`PUSH1 2, PUSH1 3, ADD, STOP`) et on vérifie la preuve.

### Ajouté
- Crate `crates/prover/` — programme guest SP1 (cible `riscv32im-succinct-zkvm-elf`) :
  lit le bytecode via `sp1_zkvm::io::read`, exécute `evm::run`, et commit le top
  de stack final (32 octets big-endian) comme valeurs publiques de la preuve.
- Crate `crates/host/` — driver SP1 (API bloquante `sp1-sdk` 6.2.2) :
  - `build.rs` compile l'ELF guest via `sp1_build::build_program`.
  - sous-commande `prove <bytecode.hex>` → preuve dans `target/proof.bin` + métriques
    (cycles RISC-V, temps de génération, taille).
  - sous-commande `verify <proof.bin>` → re-dérive la vk depuis l'ELF embarqué et vérifie.
- `programs/add.hex` (`600260030100`).
- Section « Build & test » et table de métriques remplies dans le README.

### Choix techniques
- **`evm` reste sans dépendance SP1.** Les crates `prover`/`host` sont **exclus du
  workspace racine** (`exclude` dans `Cargo.toml`) : `cargo test -p evm` et la CI
  (qui n'a pas la toolchain SP1) ne les compilent jamais. Conséquence : on lance
  `prove`/`verify` depuis `crates/host`, pas via `--bin host` à la racine.
- Le guest dépend de `evm` avec `default-features = false` (no_std), validant que
  l'interpréteur est SP1-ready ; `evm` était déjà no_std-compatible (rien à refacto).
- **Preuve « core »** (pas de compression Groth16 ici) : suffisant pour le critère
  d'acceptation, beaucoup plus rapide à générer, et vérifiable localement.
- `.gitignore` : `/target` → `target/` pour ignorer aussi `crates/*/target/` (artefacts
  SP1 + preuve de ~2.7 Mo).

### Environnement
- Toolchain SP1 installée via `sp1up` (cargo-prove sp1 6.2.2, toolchain `succinct`
  rustc 1.93.0-dev). `protoc` requis par `sp1-prover-types` (installé hors root, exposé
  via `PROTOC`).

### Validation
- Critère d'acceptation : `host prove ../../programs/add.hex` → preuve écrite ;
  `host verify target/proof.bin` → **OK**.
- Top de stack committé = `0x…05` (2 + 3 = 5) ✓.
- Métriques (prouveur CPU local) : ~6 777 cycles, génération ~51 s, preuve ~2.65 Mo,
  vérification ~93 ms.
- `cargo test -p evm` (52 tests), `clippy --all-features -D warnings`, `fmt --check`
  à la racine : toujours verts (l'exclusion ne touche pas la CI).

## [v0.8-swap1] — 2026-05-29

Phase 1 — échange de la stack. Dernier opcode de la phase 1 : l'interpréteur
exécute désormais de petits programmes mêlant arithmétique et manipulation de
stack.

### Ajouté
- Opcode `SWAP1` (`0x90`) : échange le top et le second élément de la stack.
- Helper `swap(state, n)` généralisé (échange le top avec l'élément à `n` positions
  sous lui), prêt pour `SWAP2`..`SWAP16` (v0.11).
- Tests natifs (`swap1.rs`) : `[1,2]`→`[2,1]`, n'échange que les deux du haut
  (`[1,2,3]`→`[1,3,2]`), underflow sur stack vide et sur un seul élément.
- Extension de l'oracle revm (`swap1_matches_revm`, `swap1_underflow_matches_revm`).
- **Programme de démo** `programs/arith.hex` (critère de fin de Phase 1) combinant
  `PUSH1`, `ADD`, `MUL`, `DUP1`, `SWAP1` → stack finale `[20, 7, 20]`, validé contre
  revm par le test `arith_demo_program_matches_revm` (décodage hex du fichier).

### Choix techniques
- **Cas d'erreur `SWAP1`** : on asserte la variante exacte `StackUnderflow` côté
  revm (contrairement à `DUP` qui renvoie `StackOverflow`) — revm renvoie bien
  `StackUnderflow` pour un `SWAP` sur stack trop courte.

### Validation
- `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, `cargo test --all`
  (52 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : `SWAP1` (`[1,2]`→`[2,1]` et `[1,2,3]`→`[1,3,2]`) → stack identique ;
  underflow rejeté des deux côtés ; programme de démo `arith.hex` → stack identique.

## [v0.7-dup1] — 2026-05-28

Phase 1 — duplication de la stack.

### Ajouté
- Opcode `DUP1` (`0x80`) : duplique le top de la stack.
- Helper `dup(state, n)` généralisé (`n`-ième élément depuis le top), prêt pour `DUP2`..`DUP16` (v0.11) ; applique la limite 1024 → `StackOverflow`.
- Tests natifs (`dup1.rs`) : `[5]`→`[5,5]`, ne duplique que le top, underflow, overflow à 1024 éléments.
- Extension de l'oracle revm (`dup1_matches_revm`, `dup1_underflow_matches_revm`).

### Choix techniques
- **Convention oracle pour les cas d'erreur** : on n'asserte plus la variante exacte de `InstructionResult` de revm, seulement que revm **et** nous rejetons le programme. Raison : revm renvoie `StackOverflow` (et non `StackUnderflow`) pour un `DUP` sur stack trop courte — son `Stack::dup` fusionne les deux bornes. Notre `StackUnderflow` reste le bon sémantique EVM.

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (45 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : `DUP1` → stack identique ; underflow rejeté des deux côtés.

## [v0.6-mul] — 2026-05-28

Phase 1 — multiplication.

### Ajouté
- Opcode `MUL` (`0x02`) : `push((a * b) mod 2²⁵⁶)` (`U256::wrapping_mul`).
- Tests natifs (`mul.rs`) : `3*4=12`, `x*0=0`, `x*1=x`, overflow `2¹²⁸ * 2¹²⁸ = 0` (stack manuelle), underflow.
- Extension de l'oracle revm (`mul_matches_revm` sur 4 programmes, `mul_underflow_matches_revm`).

### Notes
- Gas de `MUL` = 5 (vs 3 pour ADD/SUB) — sans effet tant que le compteur de gas n'existe pas (v0.21), noté pour mémoire.

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (39 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : 4 programmes `MUL` (dont `255*2=510`) → stack identique ; underflow des deux côtés.

## [v0.5-sub] — 2026-05-28

Phase 1 — soustraction.

### Ajouté
- Opcode `SUB` (`0x03`) : `top - second` en wrapping (`U256::wrapping_sub`).
- Tests natifs (`sub.rs`) : `5-3=2`, wrapping `3-5=MAX-1`, `7-7=0`, underflow.
- Extension de l'oracle revm (`sub_matches_revm` sur 4 programmes, `sub_underflow_matches_revm`).

### Choix techniques
- **Ordre des opérandes** : `a = top` (premier `pop`), `b = second`, résultat `a - b`. C'est la source d'erreur classique ; le test `3-5=MAX-1` verrouille l'ordre.

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (32 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : 4 programmes `SUB` (dont le wrapping `3-5`) → stack identique ; underflow des deux côtés.

## [v0.4-add] — 2026-05-28

Phase 1 — première opération arithmétique. Jalon symbolique : « 2 + 3 = 5 ».

### Ajouté
- Opcode `ADD` (`0x01`) : pop `a` (top) et `b`, push `a + b` mod 2²⁵⁶ (`U256::wrapping_add`).
- Helper `pop2()` → `(top, second)`.
- Tests natifs (`add.rs`) : `2+3=5`, wrapping `MAX+1=0` et `(MAX-2)+5=2` (stack construite à la main en attendant `PUSH32` en v0.10), underflows.
- Extension de l'oracle revm (`add_matches_revm` sur 4 programmes, `add_underflow_matches_revm`).

### Choix techniques
- **Modulo 2²⁵⁶ via `wrapping_add`** (jamais `checked_add` qui panique).
- Test `unknown_opcode_errors` migré de `0x01` (désormais ADD) vers `0x21` (opcode EVM non assigné, stable).

### Validation
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all` (26 tests) : OK.
- Build `--no-default-features` (`no_std`) : OK.
- `revm` : 4 programmes `ADD` (dont `255+1=256`) → stack identique ; `ADD` avec un seul opérande → underflow des deux côtés.

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
