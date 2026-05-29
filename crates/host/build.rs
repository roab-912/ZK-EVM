//! Compile the SP1 guest (`crates/prover`) to its zkVM ELF at build time.
//! The ELF is then embedded in the host via `include_elf!("prover")`.
fn main() {
    sp1_build::build_program("../prover");
}
