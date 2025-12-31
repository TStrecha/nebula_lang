use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use nebula_codegen::generate_llvm_code;
use nebula_ir_gen::builder::IRModuleBuilder;
use nebula_lexer::tokenizer::tokenize;
use nebula_parser::parser::AstParser;
fn main() {
    let source_code = fs::read_to_string("example/test.neb").unwrap();

    let tokens = tokenize(&source_code);
    let mut parser = AstParser::new(tokens);

    let mut ast = vec![];
    loop {
        let next_item = parser.next_item();

        if let Some(next_item) = next_item {
            ast.push(next_item);
        } else {
            break;
        }
    }

    println!(" ===== AST ===== ");
    println!("{:?}", &ast);

    let mut ir_module_builder = IRModuleBuilder::new(ast);
    ir_module_builder.build();

    println!(" ===== IR ====== ");
    println!("{:?}", ir_module_builder.module);

    let backend_code = generate_llvm_code(ir_module_builder.module);

    let mut file = File::create("out/program.ll").unwrap();
    file.write_all(backend_code.as_bytes()).unwrap();

    if false {
        Command::new("clang")
            .arg("out/program.ll")
            .arg("-isysroot")
            .arg(env::var("SDKROOT").unwrap())
            .arg("-o")
            .arg("out/program")
            .status()
            .expect("failed to execute clang");
    }
}
