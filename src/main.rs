use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use nebula_lexer::tokenizer::tokenize;
use nebula_parser::{build_ast, build_tst};
use nebula_ir_gen::generate_ir_module;
use nebula_codegen::{generate_llvm_ir};

fn main() {
    let source_code = fs::read_to_string("example/test.neb").unwrap();

    let tokens = tokenize(&source_code);
    let ast = build_ast(tokens);
    let tst = build_tst(ast);

    println!(" ===== TST ===== ");
    for item in tst.items() {
        println!("{:?}", &item);
    }
    let ir_module = generate_ir_module(tst);

    println!(" ===== IR ====== ");
    println!(" -- GLOBAL -- ");
    for global in ir_module.globals() {
        println!("{:?}", global);
    }

    println!(" -- INSTR -- ");
    for instruction in ir_module.instructions() {
        println!("{:?}", instruction);
    }

    let backend_code = generate_llvm_ir(vec![ir_module]);

    let mut file = File::create("out/program.ll").unwrap();
    file.write_all(backend_code.as_bytes()).unwrap();

    if true {
        Command::new("clang")
            .arg("out/program.ll")
            .arg("-o")
            .arg("out/program")
            .status()
            .expect("failed to execute clang");
    }
}
