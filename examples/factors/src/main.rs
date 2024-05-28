use jolt_sdk::{host, Jolt, JoltPreprocessing, RV32IJoltVM, F, PCS};

pub fn main() {
    let p = 15;
    let (a, b) = (5, 3);

    let mut prog = host::Program::new("factors-guest");
    let (bytecode, memory_init) = prog.decode();

    // Preprocess the program, prove its execution.
    dbg!("preprocessing");
    let preproc: JoltPreprocessing<4, F, PCS> =
        RV32IJoltVM::preprocess(bytecode.clone(), memory_init, 1 << 20, 1 << 20, 1 << 20);

    // let (prog, preproc) = guest::preprocess_correct_factors();
    dbg!("tracing");

    prog.set_input(&p);
    prog.set_input(&a);
    prog.set_input(&b);

    // let (output, proof) = guest::prove_correct_factors(prog, preproc.clone(), p, a, b);

    let (io_device, trace) = prog.trace();

    dbg!("proving");
    let (proof, commitments, debug_info) = RV32IJoltVM::prove(io_device, trace, preproc.clone());

    let (proof_p, proof_a, proof_b): (i32, i32, i32) =
        postcard::from_bytes(&proof.program_io.inputs).unwrap();

    let prog_output: bool = postcard::from_bytes(&proof.program_io.outputs).unwrap();
    dbg!((proof_p, proof_a, proof_b));
    dbg!(prog_output);

    // Transmit the proof to the verifier
    // ...

    // Load the proof
    // ...

    // Preprocess the program on the verifier side, verify the proof.

    dbg!("verifying");
    let is_valid = RV32IJoltVM::verify(preproc, proof, commitments, debug_info).is_ok();

    dbg!(is_valid);
}
