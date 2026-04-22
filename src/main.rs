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
    let file_name = args.get(1);
    let mut source_file = match File::open(file_name){
        Ok(source_file) => source_file,
        Err(err) => panic!("File Could not be opened: {}", err),
    };
    let mut source = String::new();
    match source_file.read_to_string(&mut source){
        Ok(source) => source,
        Err(err) => panic!("Instructions could not be read: {}", err),
    };
    dbg!(args);
    let source = r#"
def greet(name)
    puts "Hello, " + name + "!";
end;

greet("world");

x = 10;
y = 3;
puts x + y;

def factorial(n)
    if n < 2 then
        return 1;
    end;
    return n * factorial(n - 1);
end;

result = factorial(5);
puts result;

i = 1;
while i < 6 do
    puts i;
    i = i + 1;
end;

if 10 > 5 then
    puts "ten is greater";
else
    puts "this won't print";
end;
"#;

    let parser = shavenscript::ProgramParser::new();
    let program = parser.parse(source).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });
        let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}