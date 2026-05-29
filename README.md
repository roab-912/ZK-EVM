# Mini ZK-EVM (PoC)

> Construction incrémentale d'un ZK-EVM simplifié en Rust : un interpréteur de bytecode Ethereum (sous-ensemble) couplé à un système de génération et vérification de preuves cryptographiques d'exécution correcte.

Projet de démonstration des compétences en **cryptographie appliquée**, **compilation** et **architecture systèmes**. La roadmap complète est dans [`.features/ZK EVM Roadmap.md`](./.features/ZK%20EVM%20Roadmap.md). Le détail développeur de chaque version est dans [`.features/`](./.features/), un fichier par tag git.

---

## Philosophie

- **Incrémental** — un opcode = un commit = un tag git. Chaque version est testable et démontrable.
- **Mesurable** — chaque phase a des critères d'acceptation clairs (tests qui passent, preuve qui vérifie).
- **Honnête** — on documente ce qui est implémenté, ce qui est simplifié, et ce qui est ignoré.
- **Comparable** — validation continue contre [`revm`](https://github.com/bluealloy/revm), l'interpréteur EVM de référence en Rust.

### Ce qu'on construit

Un ZK-EVM **simplifié mais réel** : interpréteur EVM + preuves zkVM (SP1), puis migration partielle vers circuits custom (Halo2).

### Ce qu'on ne construit pas

Un ZK-EVM Type 1 / production-ready, un rollup déployable sur mainnet, ni une couverture EVM à 100%.

---

## Stack technique

| Composant | Choix | Justification |
|---|---|---|
| Langage | **Rust** | Écosystème ZK massivement en Rust, perf, sûreté mémoire |
| zkVM (phase initiale) | **SP1** (Succinct) | API simple, doc fraîche, code Rust standard |
| zkVM (alternative) | RISC0 | Plus mature mais plus verbeux |
| Framework circuit (phase avancée) | **Halo2** (PSE fork) ou **Plonky3** | Migration vers circuits custom |
| Integer ops | `ruint` ou `primitive-types` | U256 robuste et testé |
| Référence EVM | `revm` | Oracle pour valider nos exécutions |
| Tests Solidity | `foundry` (`forge`) | Compilation de contrats jouets, extraction du bytecode |
| Hash dans tests | `tiny-keccak` | Avant d'avoir notre propre implémentation |

---

## Structure du projet

```
mini-zkevm/
├── Cargo.toml                    # Workspace
├── README.md
├── .features/                    # Roadmap + détail par version
│   ├── ZK EVM Roadmap.md
│   ├── v0.1-stop.md
│   ├── v0.2-push1.md
│   └── ...
├── crates/
│   ├── evm/                      # Interpréteur EVM pur (sans ZK)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── opcodes.rs        # Enum + décodage
│   │   │   ├── interpreter.rs    # Boucle d'exécution
│   │   │   ├── state.rs          # EvmState (stack, memory, pc...)
│   │   │   ├── memory.rs         # Mémoire expandable
│   │   │   ├── storage.rs        # Plus tard : MPT
│   │   │   ├── keccak.rs         # Implémentation Keccak-256
│   │   │   ├── errors.rs         # EvmError
│   │   │   └── gas.rs            # Comptage du gas
│   │   └── tests/
│   ├── prover/                   # Programme SP1 (cible RISC-V)
│   │   └── src/main.rs
│   ├── host/                     # Génération et vérification de preuve
│   │   └── src/main.rs
│   └── circuits/                 # Phase avancée : circuits Halo2
│       └── src/
├── programs/                     # Bytecodes de test
│   ├── add.hex
│   ├── fib.hex
│   └── erc20_transfer.hex
└── benches/                      # Benchmarks de perf
```

### Séparation EVM ↔ prover

Le point d'architecture le plus important. Le crate `evm/` ne dépend de rien lié à SP1. Conséquences :
- Tests de l'EVM nativement (`cargo test`, rapide, < 1 s)
- SP1 lancé ponctuellement, quand on veut une preuve (lent, plusieurs minutes)
- Migration future vers Halo2 sans toucher au crate `evm/`

---

## Roadmap (vue d'ensemble)

| Phase | Versions | Thème | Difficulté ZK |
|---|---|---|---|
| 0 | setup | Workspace, CI | ⭐ |
| 1 | v0.1 – v0.8 | Opcodes basiques (STOP, PUSH1, ADD, …) | ⭐ |
| 2 | v0.9 | Branchement SP1 — première preuve | ⭐⭐ |
| 3 | v0.10 – v0.17 | Arithmétique & logique étendues | ⭐ |
| 4 | v0.18 – v0.21 | Mémoire & contrôle de flux | ⭐⭐ |
| **5** | **v0.22** | **KECCAK-256** (le gros saut) | ⭐⭐⭐⭐ |
| 6 | v0.23 – v0.27 | Environnement d'exécution | ⭐⭐ |
| 7 | v0.28 – v0.32 | Storage + SMT | ⭐⭐⭐⭐ |
| 8 | v0.33 | ECRECOVER (précompilé) | ⭐⭐⭐⭐ |
| 9 | v0.34 – v0.40 | La famille CALL | ⭐⭐⭐⭐ |
| 10 | v0.41 – v0.46 | Long tail des opcodes | ⭐⭐ |
| 11 | — | Circuits custom Halo2 | ⭐⭐⭐⭐⭐ |

**Paliers de complétion :**
- 🥉 **Bronze** (4-6 semaines) : Phases 0-4. ~55 opcodes, premières preuves SP1.
- 🥈 **Argent** (3-4 mois) : Phases 0-8. Keccak + SMT + ECRECOVER. Transaction ERC-20 prouvée.
- 🥇 **Or** (6-9 mois) : Phases 0-10 + migration partielle Halo2.

---

## Convention de versionnement

- **Tags git** : `v0.X-<opcode>` ou `v0.X-<feature>` (ex : `v0.4-add`, `v0.22-keccak`)
- **Branches** : `main` toujours fonctionnelle, branches `feature/<opcode>` pour le WIP
- **Critère de merge** : tous les tests passent, validation contre `revm` sur au moins un programme de test

Chaque version a un fichier dédié dans [`.features/`](./.features/) qui contient : objectif, spec des opcodes, plan d'implémentation, tests, critères d'acceptation, pièges connus.

---

## Build & test

```bash
# Tests de l'EVM (rapide, < 1 s) — crate `evm` uniquement
cargo test -p evm
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Génération de preuve SP1 (à partir de v0.9). Prérequis : la toolchain SP1
(`curl -L https://sp1up.succinct.xyz | bash && sp1up`) et `protoc`. Les crates
`host`/`prover` vivent **hors** du workspace (pour que `evm` et la CI restent
sans dépendance SP1) ; on les lance donc depuis `crates/host` :

```bash
cd crates/host
cargo run --release -- prove  ../../programs/add.hex   # → target/proof.bin + métriques
cargo run --release -- verify target/proof.bin         # → OK
```

---

## Métriques (à remplir à partir de v0.9)

Preuve **core** SP1 (sans compression Groth16), prouveur CPU local (16 cœurs),
programme `programs/add.hex` (`PUSH1 2, PUSH1 3, ADD, STOP`) :

| Version | Temps gen. preuve | Taille preuve | Temps vérif. | Cycles RISC-V |
|---|---|---|---|---|
| v0.9-sp1 | ~51 s | ~2.65 MB | ~93 ms | 6 777 |
| v0.17 | — | — | — | — |
| v0.22-keccak | — | — | — | — |

---

## Ressources

- [Vitalik — *The different types of ZK-EVMs*](https://vitalik.eth.limo/general/2022/08/04/zkevm.html)
- [Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [evm.codes](https://www.evm.codes/) — référence interactive des opcodes
- [`revm`](https://github.com/bluealloy/revm), [SP1 examples](https://github.com/succinctlabs/sp1/tree/main/examples)
- [Scroll zkEVM circuits](https://github.com/scroll-tech/zkevm-circuits), [Polygon zkEVM](https://github.com/0xPolygonHermez)

---

## License and reuse

**Copyright © Rémi Barbier. All rights reserved.**

This repository is published for the purposes of transparency, peer review,
and reproducibility. **No open-source licence is granted.** The absence of a
permissive licence (such as MIT, Apache-2.0, BSD, or GPL) is intentional:
this work is *not* free or open-source software in the legal sense of those
terms, and the author retains the entirety of the rights conferred by the
French *Code de la propriété intellectuelle* and, where applicable, the
Berne Convention.

Permitted, without prior written consent:

* viewing the source on this hosting platform;
* citing this work in academic or professional publications, with proper
  attribution to the author and a stable reference to this repository;
* executing the code locally for the **sole purpose of personal study,
  scientific verification, or reproduction of any results published herein**,
  in a private, non-commercial setting;
* quoting short excerpts (limited and non-substantial, in the sense of *fair
  use* / *courte citation*) within a properly attributed publication.

Not permitted, absent prior written authorisation from the author:

* redistribution, in source or compiled form, in whole or in part;
* the production of derivative works (forks, ports, refactorings,
  translations, integrations into other codebases);
* any commercial use, whether direct or indirect, including in
  proof-of-concept pilots, internal tooling, or hosted services;
* the use of this work, in whole or in part, to train, fine-tune, or
  otherwise inform the behaviour of automated systems (machine-learning
  models, code-generation models, retrieval-augmented systems);
* the removal or alteration of authorship, copyright, or licensing notices.

Requests for any use beyond the permissions enumerated above must be
addressed in writing to the author; such requests will be considered on a
case-by-case basis.

This notice is provided in good faith and is not intended to be exhaustive;
no clause of this section shall be construed to grant, by implication,
estoppel, or otherwise, any licence under any patent, trademark, copyright,
or other intellectual property right, except as explicitly stated above.

