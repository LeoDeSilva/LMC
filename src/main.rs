use clap::{Parser as ArgsParser};
mod machine;
mod assembler;

#[derive(ArgsParser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}


#[derive(ArgsParser)]
enum Subcommand {
    Assemble {
        path: std::path::PathBuf,
        out: std::path::PathBuf,
    }, 

    Run {
        path: std::path::PathBuf,
    }
}

fn run(path: std::path::PathBuf) {
    let program = std::fs::read(path).expect("could not read from file");

    let mut m = machine::machine::Machine::new();
    m.load(program);
    m.emulate();
}



fn assemble(path: std::path::PathBuf, out: std::path::PathBuf) {
    let content = std::fs::read_to_string(path).expect("could not read file");

    let mut l = assembler::lexer::Lexer::new(content.chars().collect());
    let tokens: Vec<assembler::lexer::Token> = l.lex();

    let mut p = assembler::parser::Parser::new(tokens);
    let (program, symbol_table) = p.parse();

    let mut c = assembler::compiler::Compiler::new(program, symbol_table);
    let bin: Vec<u8> = c.compile();
    std::fs::write(out, bin).unwrap();
}

fn main () {
    let args = Cli::parse();    
    match args.subcommand {
        Subcommand::Assemble { path, out } => {
            assemble(path, out);
        }

        Subcommand::Run { path } => {
            run(path);
        }
    }
}