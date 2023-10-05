use clap::Parser as ClapParser;
mod machine;
mod compiler;
mod assembler;

#[derive(ClapParser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}


#[derive(ClapParser)]
enum Subcommand {
    Assemble {
        path: std::path::PathBuf,
        out: std::path::PathBuf,
    }, 

    Emulate {
        path: std::path::PathBuf,
    },

    Compile {
        path: std::path::PathBuf,
        out: std::path::PathBuf,
    },

    Semicompile {
        path: std::path::PathBuf,
    },

    Run {
        path: std::path::PathBuf,
    }
}

fn emulate(program: Vec<u8>) {
    let mut m = machine::machine::Machine::new();
    m.load(program);
    m.emulate();
}

fn assemble(path: std::path::PathBuf) -> Vec<u8> {
    let content = std::fs::read_to_string(path).expect("could not read file");

    let mut l = assembler::lexer::Lexer::new(content.chars().collect());
    let tokens: Vec<assembler::lexer::Token> = l.lex();

    let mut p = assembler::parser::Parser::new(tokens);
    let (program, symbol_table) = p.parse();

    let mut c = assembler::assembler::Compiler::new(program, symbol_table);
    c.compile()
}

fn compile(program: String) -> String {
    let mut l = compiler::lexer::Lexer::new(program.chars().collect());
    let tokens = l.lex();

    let mut p = compiler::parser::Parser::new(tokens);
    let ast = p.parse();

    let mut c = compiler::compiler::Compiler::new();
    let out = c.compile(ast);
    println!("{}", out);
    out
}

fn main () {
    let args = Cli::parse();    
    match args.subcommand {
        Subcommand::Assemble { path, out } => {
            let bin: Vec<u8> = assemble(path);
            std::fs::write(out, bin).unwrap();
        }

        Subcommand::Emulate { path } => {
            let program = std::fs::read(path).expect("could not read from file");
            emulate(program);
        }

        Subcommand::Run { path } => {
            let program: Vec<u8> = assemble(path);
            emulate(program);
        }

        Subcommand::Compile { path, out } => {
            let content = std::fs::read_to_string(path).expect("could not read file ");
            std::fs::write(out, compile(content)).unwrap();
        }

        Subcommand::Semicompile { path } => {
            let content = std::fs::read_to_string(path).expect("could not read file ");
            let mut l = assembler::lexer::Lexer::new(compile(content).chars().collect());
            let tokens: Vec<assembler::lexer::Token> = l.lex();

            let mut p = assembler::parser::Parser::new(tokens);
            let (program, symbol_table) = p.parse();

            let mut c = assembler::assembler::Compiler::new(program, symbol_table);
            let program: Vec<u8> = c.compile();

            emulate(program);
        }
    }
}