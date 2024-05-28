use jolt_sdk::{Jolt, RV32IJoltVM};

pub fn main() {
    let p = 15;
    let (a, b) = (5, 3);

    // Preprocess the program, prove its execution.
    dbg!("preprocessing");
    let (prog, preproc) = guest::preprocess_correct_factors();
    dbg!("proving");
    let (output, proof) = guest::prove_correct_factors(prog, preproc.clone(), p, a, b);

    let (proof_p, proof_a, proof_b): (i32, i32, i32) =
        postcard::from_bytes(&proof.proof.program_io.inputs).unwrap();
    let proof_output: bool = postcard::from_bytes(&proof.proof.program_io.outputs).unwrap();
    dbg!((proof_p, proof_a, proof_b));
    dbg!(proof_output);

    // Transmit the proof to the verifier
    // ...

    // Load the proof
    // ...

    // Preprocess the program on the verifier side, verify the proof.
    // dbg!("preprocessing");
    // let (_, preproc) = guest::preprocess_correct_factors();

    dbg!("verifying");
    let is_valid = RV32IJoltVM::verify(preproc, proof.proof, proof.commitments, None).is_ok();

    dbg!(output);
    dbg!(is_valid);
}
