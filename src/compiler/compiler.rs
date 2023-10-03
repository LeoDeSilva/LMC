use std::collections::HashMap;

use crate::compiler::node::Node;
use crate::compiler::lexer::Token;

pub struct Compiler {
    constants: HashMap<i32, String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { constants: HashMap::new() }
    }

    pub fn compile(&mut self, ast: Node) -> String {
        let mut out = self.compile_node(ast);
        for (value, label) in &self.constants {
            out = out + &format!("{label} dat {value}\n");
        }
        out
    }

    fn compile_node(&mut self, node: Node) -> String {
        match node {
            Node::BLOCK(statements) => { self.compile_block(*statements) }
            Node::DECLARATION(identifier, expression) => { self.compile_declaration(*identifier, *expression) }
            Node::INFIX(lhs, op, rhs) => { self.compile_infix(*lhs, op, *rhs) }

            Node::NUMBER(value) => { self.compile_number(value) },

            _ => { panic!("Unexpected node found in compile_node(), got: {:?}", node)}
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
        format!("{identifier} dat 0\n{expression} \nsta {identifier}\n")

    }

    fn compile_infix(&mut self, lhs: Node, op: Token, rhs: Node) -> String {
        String::new()
    }

    fn compile_number(&mut self, value: i32) -> String {
        if self.constants.contains_key(&value) {
            "lda ".to_owned() + &self.constants.get(&value).unwrap().clone()
        } else {
            self.constants.insert(value, "_".to_owned() + &value.to_string());
            "lda ".to_owned() + &self.constants.get(&value).unwrap().clone()
        }
    }
}
