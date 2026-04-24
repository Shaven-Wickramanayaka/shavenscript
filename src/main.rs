pub mod ast;
pub mod interpreter;

use core::panic;
use std::env;
use std::fs::File;
use std::io::Read;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    shavenscript
);

use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut source_file = match File::open(args[1].clone()){
        Ok(source_file) => source_file,
        Err(err) => panic!("File Could not be opened: {}", err),
    };
    let mut source = String::new();
    match source_file.read_to_string(&mut source){
        Ok(source) => source,
        Err(err) => panic!("Instructions could not be read: {}", err),
    };
    let parser = shavenscript::ProgramParser::new();
    let program = parser.parse(source.as_str()).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });
        let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}