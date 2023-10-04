use std::collections::HashMap;

use crate::compiler::node::Node;
use crate::compiler::lexer::Token;

pub struct Compiler {
    constants: HashMap<i32, String>,
    variables: HashMap<String, String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { constants: HashMap::new(), variables: HashMap::new(), }
    }

    pub fn compile(&mut self, ast: Node) -> String {
        let mut out = self.compile_node(ast);
        for (value, label) in &self.constants {
            out = out + &format!("{label} dat {value}\n");
        }
        println!("{}", out);
        out
    }

    fn compile_node(&mut self, node: Node) -> String {
        match node {
            Node::BLOCK(statements) => { self.compile_block(*statements) }
            Node::DECLARATION(identifier, expression) => { self.compile_declaration(*identifier, *expression) }
            Node::INFIX(lhs, op, rhs) => { self.compile_infix(*lhs, op, *rhs) }

            Node::NUMBER(value) => { "lda ".to_owned() + &self.compile_number_literal(value) },
            Node::IDENTIFIER(identifier) => { "lda ".to_owned() + &self.compile_identifier_literal(identifier) },

            _ => { panic!("Unexpected node found in compile_node(), got: {:?}", node)}
        }
   }

   fn compile_atom(&mut self, atom: Node) -> String {
        match atom {
            Node::NUMBER(value) => { self.compile_number_literal(value) },
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

    fn compile_declaration(&mut self, identifier_node: Node, expression_node: Node) -> String {
        let identifier = if let Node::IDENTIFIER(identifier) = &identifier_node { identifier } else { panic!("Parser error, expected identifier type: IDENTIFIER, got: {:?}", identifier_node) };
        let expression = self.compile_node(expression_node);
        self.variables.insert(identifier.clone(), identifier.clone());
        format!("{identifier} dat 0\n{expression}\nsta {identifier}\n")

    }

    fn compile_infix(&mut self, lhs_node: Node, op_tok: Token, rhs_node: Node) -> String {
        let lhs = self.compile_node(lhs_node);
        let rhs = self.compile_atom(rhs_node);
        format!("{lhs}\n{:?} {rhs}", op_tok)
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
        if !self.variables.contains_key(&identifier) {
            panic!("Cannot reference uninitialised variable: {}", identifier);
        }

        identifier
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
                Box::new(Node::IDENTIFIER(String::from("x"))), 
                Box::new(Node::NUMBER(1)),
            )]
        ))), 

        String::from("x dat 0\nlda _1\nsta x\n_1 dat 1\n")
        );
    }

    #[test]
    fn test_compile_infix() {
        let mut c = Compiler::new();
        let out: String = c.compile(Node::BLOCK(Box::new(vec![
            Node::DECLARATION(
                Box::new(Node::IDENTIFIER(String::from("x"))), 
                Box::new(Node::INFIX(
                    Box::new(Node::NUMBER(1)), 
                    Token::ADD, 
                    Box::new(Node::NUMBER(2)))),
            )]
        )));

        assert_eq!(out[0..28], 
        String::from("x dat 0\nlda _1\nADD _2\nsta x\n"));
    }
}