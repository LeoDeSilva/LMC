use std::collections::HashMap;

use crate::compiler::node::Node;
use crate::compiler::lexer::Token;

pub struct Compiler {
    constants: HashMap<i32, String>,
    variables: HashMap<String, String>,
    libraries: HashMap<String, std::path::PathBuf>
}

impl Compiler {
    pub fn new() -> Self {
        let libraries: HashMap<String, std::path::PathBuf> = [
            ("std".to_string(), std::path::PathBuf::from("src/compiler/linker/std.lmasc"))
        ].iter().cloned().collect();
        Compiler { constants: HashMap::new(), variables: HashMap::new(), libraries: libraries }
    }


    pub fn compile(&mut self, ast: Node) -> String {
        let mut out = self.compile_node(ast);
        for (value, label) in &self.constants {
            out = out + &format!("{label} dat {value}\n");
        }
        "call _main\nhlt\n".to_owned() + &out 
    }


    fn compile_node(&mut self, node: Node) -> String {
        match node {
            Node::BLOCK(statements) => { self.compile_block(*statements) }
            Node::DECLARATION(identifier, expression) => { self.compile_declaration(identifier, *expression) }
            Node::ASSIGNMENT(identifier, expression) => { self.compile_assignment(identifier, *expression) }
            Node::INFIX(lhs, op, rhs) => { self.compile_infix(*lhs, op, *rhs) }
            Node::INVOCATION(id, args) => { self.compile_invocation(id, *args) }
            Node::LIBRARY(library) => { self.compile_library(library) }
            Node::FUNCTION(id, args, block) => { self.compile_function(id, args, *block) }
            Node::RETURN(expression) => { self.compile_return(*expression) }
            Node::IF(conditionals, alternative) => { self.compile_if(*conditionals, *alternative) }
            Node::HALT() => { "hlt\n".to_string() }

            Node::NUMBER(value) => { "lda ".to_owned() + &self.compile_number_literal(value) }
            Node::IDENTIFIER(identifier) => { "lda ".to_owned() + &self.compile_identifier_literal(identifier) }

            _ => { panic!("Unexpected node found in compile_node(), got: {:?}", node)}
        }
   }


   fn compile_atom(&mut self, atom: Node) -> String {
        match atom {
            Node::NUMBER(value) => { self.compile_number_literal(value) },
            Node::IDENTIFIER(id) => { self.compile_identifier_literal(id) },
            _ => { panic!("Unexpected node found in compile_node(), got: {:?}", atom)}
        }
   }


    fn compile_block(&mut self, statements: Vec<Node>) -> String {
        let mut out: String = String::new();
        for node in statements.iter() {
            out.push_str(&self.compile_node(node.clone()));
        }

        out
    }


    fn compile_declaration(&mut self, identifier: String, expression_node: Node) -> String {
        let expression = self.compile_node(expression_node);
        self.variables.insert(identifier.clone(), identifier.clone());
        format!("{identifier} dat 0\n{expression}\nsta {identifier}\n")
    }

    
    fn compile_assignment(&mut self, identifier: String, expression_node: Node) -> String {
        let expression = self.compile_node(expression_node);
        format!("{expression}\nsta {identifier}\n")
    }


    fn compile_infix(&mut self, lhs_node: Node, op_tok: Token, rhs_node: Node) -> String {
        let lhs = self.compile_node(lhs_node);
        let rhs = self.compile_atom(rhs_node);

        let mut op = op_tok;
        match op {
            Token::EE | Token::NE | Token::GT | Token::GTE | Token::LT | Token::LTE  => { 
                //TODO: BGT, BLT, BLZ implement
                op = Token::SUB;
                // additional return, SUB, B<expr>
            }
            _ => {}
        }

        format!("{lhs}\n{:?} {rhs}", op)
    }


    fn compile_number_literal(&mut self, value: i32) -> String {
        if self.constants.contains_key(&value) {
            self.constants.get(&value).unwrap().clone()
        } else {
            self.constants.insert(value, "_".to_owned() + &value.to_string());
            self.constants.get(&value).unwrap().clone()
        }
    }


    fn compile_identifier_literal(&mut self, identifier: String) -> String {
        identifier
    }


    fn compile_library(&mut self, library: String) -> String {
        let lib_path = self.libraries.get(&library).expect(&format!("No library exists with name: {}", library));
        let lib_content = std::fs::read_to_string(lib_path).expect("could not read library");
        lib_content + "\n"
    }


    fn compile_invocation(&mut self, identifier: String, args: Vec<Node>) -> String {
        let mut arg_counter = 0;
        let mut arg_out: String = String::new();
        for arg in args {
            let arg_id = "_p".to_owned() + &arg_counter.to_string();
            arg_out.push_str(&self.compile_node(Node::DECLARATION(
                arg_id, 
                Box::new(arg),
            )));

            arg_counter += 1;
        }

        format!("{arg_out}call {identifier}\nlda _ret\n")
    }


    fn compile_function(&mut self, identifier: String, args: Vec<String>, block: Node) -> String {
        let mut args_out: String = String::new();
        let mut arg_counter = 0;

        for arg in args {
            self.variables.insert(arg.clone(), arg.clone());
            let arg_id = "_p".to_owned() + &arg_counter.to_string();

            args_out.push_str(&self.compile_node(Node::DECLARATION(
                arg, 
                Box::new(Node::IDENTIFIER(arg_id))
            )));

            arg_counter += 1;
        }

        format!("{identifier}\n{args_out}\n{}ret\n", self.compile_node(block))
    }

    
    fn compile_if(&mut self, conditionals: Vec<Node>, alternative: Node) -> String {
        //Loop through conditionals, branch if condition to corresponding label (& postfix each with bra .endif)
        //Follow with <else> instructions followed with bra .endif
        let mut conditional_out = String::new();
        for conditional in conditionals {
        }
        "".to_string()
    }


    fn compile_return(&mut self, expression_node: Node) -> String {
        let expr_out = self.compile_node(Node::DECLARATION(
            "_ret".to_string(), 
            Box::new(expression_node)
        ));
        format!("{expr_out}ret\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_declaration() {
        let mut c = Compiler::new();
        assert_eq!(c.compile(Node::BLOCK(Box::new(vec![
            Node::DECLARATION(
                String::from("x"), 
                Box::new(Node::NUMBER(1)),
            )]
        ))), 

        String::from("call _main\nhlt\nx dat 0\nlda _1\nsta x\n_1 dat 1\n")
        );
    }

    #[test]
    fn test_compile_infix() {
        let mut c = Compiler::new();
        let out: String = c.compile(Node::BLOCK(Box::new(vec![
            Node::DECLARATION(
                String::from("x"), 
                Box::new(Node::INFIX(
                    Box::new(Node::NUMBER(1)), 
                    Token::ADD, 
                    Box::new(Node::NUMBER(2)))),
            )]
        )));

        assert_eq!(out[0..43], 
        String::from("call _main\nhlt\nx dat 0\nlda _1\nADD _2\nsta x\n"));
    }
}