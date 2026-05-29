//! SP1 host: prove / verify the execution of a phase-1 EVM bytecode.
//!
//! ```text
//! host prove  <bytecode.hex>   # generates target/proof.bin and prints metrics
//! host verify <proof.bin>      # verifies the proof, prints verification time
//! ```
//!
//! Run from `crates/host/` (this crate is not part of the root workspace):
//! ```text
//! cargo run --release -- prove  ../../programs/add.hex
//! cargo run --release -- verify target/proof.bin
//! ```

use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use sp1_sdk::blocking::{ProveRequest, Prover, ProverClient};
use sp1_sdk::{include_elf, Elf, ProvingKey, SP1ProofWithPublicValues, SP1Stdin};

/// The guest ELF, compiled by `build.rs` (`sp1_build::build_program`).
const ELF: Elf = include_elf!("prover");

/// Default location for the generated proof.
const PROOF_PATH: &str = "target/proof.bin";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prove") => match args.get(2) {
            Some(path) => prove(path),
            None => usage(),
        },
        Some("verify") => {
            let path = args.get(2).map(String::as_str).unwrap_or(PROOF_PATH);
            verify(path);
        }
        _ => usage(),
    }
}

fn usage() -> ! {
    eprintln!("usage:");
    eprintln!("  host prove  <bytecode.hex>   # generate {PROOF_PATH}");
    eprintln!("  host verify [proof.bin]      # verify (defaults to {PROOF_PATH})");
    exit(2);
}

/// Generate a proof that running `hex_path`'s bytecode yields the committed top
/// of stack, save it to [`PROOF_PATH`], and print the metrics.
fn prove(hex_path: &str) {
    let code = read_hex_file(hex_path);
    println!("bytecode: {} ({} bytes)", to_hex(&code), code.len());

    let mut stdin = SP1Stdin::new();
    stdin.write(&code);

    let client = ProverClient::builder().cpu().build();

    // Execute first (no proof) to read the RISC-V cycle count.
    let (_public, report) = client
        .execute(ELF, stdin.clone())
        .run()
        .expect("guest execution failed");
    let cycles = report.total_instruction_count();

    let pk = client.setup(ELF).expect("setup failed");
    let vk = pk.verifying_key();

    let started = Instant::now();
    let proof = client.prove(&pk, stdin).run().expect("proving failed");
    let gen_time = started.elapsed();

    // Sanity: the proof must verify before we trust the metrics.
    client
        .verify(&proof, vk, None)
        .expect("self-verification failed");

    let top = proof.public_values.as_slice().to_vec();
    proof.save(PROOF_PATH).expect("could not save proof");
    let proof_size = fs::metadata(PROOF_PATH).map(|m| m.len()).unwrap_or(0);

    println!("--- proof generated ---");
    println!("committed top-of-stack: 0x{}", to_hex(&top));
    println!("proof written to:       {PROOF_PATH}");
    println!("--- metrics ---");
    println!("RISC-V cycles:    {cycles}");
    println!("generation time:  {gen_time:.2?}");
    println!("proof size:       {:.1} KB", proof_size as f64 / 1024.0);
}

/// Load a proof and verify it against the (deterministically derived) verifying
/// key. The host re-derives `vk` from the embedded ELF, so `verify` needs only
/// the proof file.
fn verify(proof_path: &str) {
    let client = ProverClient::builder().cpu().build();
    let pk = client.setup(ELF).expect("setup failed");
    let vk = pk.verifying_key();

    let proof = SP1ProofWithPublicValues::load(proof_path)
        .unwrap_or_else(|e| panic!("could not load proof {proof_path}: {e}"));

    let started = Instant::now();
    client
        .verify(&proof, vk, None)
        .expect("verification failed");
    let elapsed = started.elapsed();

    println!("OK — proof verified in {elapsed:.2?}");
    println!(
        "committed top-of-stack: 0x{}",
        to_hex(proof.public_values.as_slice())
    );
}

/// Read a hex file (whitespace ignored) into bytes.
fn read_hex_file(path: &str) -> Vec<u8> {
    let raw = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|e| panic!("could not read {path}: {e}"));
    decode_hex(&raw)
}

fn decode_hex(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .map(|b| (b as char).to_digit(16).expect("invalid hex digit") as u8)
        .collect();
    assert_eq!(digits.len() % 2, 0, "odd number of hex digits");
    digits.chunks_exact(2).map(|p| (p[0] << 4) | p[1]).collect()
}

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}
