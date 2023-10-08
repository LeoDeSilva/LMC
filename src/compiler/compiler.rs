use std::collections::HashMap;

use crate::compiler::node::Node;
use crate::compiler::lexer::Token;

pub struct Compiler {
    constants: HashMap<i32, String>,
    variables: HashMap<String, String>,
    libraries: HashMap<String, String>,
    label_index: i32,
}

impl Compiler {
    pub fn new() -> Self {
        let libraries: HashMap<String, String> = [
            // ("std".to_string(), std::path::PathBuf::from("src/compiler/linker/std.lmasc"))
            ("std".to_string(), include_str!("linker/std.lmasc").to_string())
        ].iter().cloned().collect();
        Compiler { constants: HashMap::new(), variables: HashMap::new(), libraries: libraries, label_index: 0 }
    }


    pub fn compile(&mut self, ast: Node) -> String {
        let mut out = self.compile_node(ast);
        for (value, label) in &self.constants {
            out = out + &format!("{label} dat {value}\n");
        }

        out += "_ret dat 0";
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
            Node::WHILE(condition, expression) => { self.compile_while(*condition, *expression) }
            Node::FOR(declaration, condition, increment, consequence) => { self.compile_for(*declaration, *condition, *increment, *consequence) }
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

        let op;
        match op_tok {
            Token::EE | Token::NE | Token::GT | Token::GTE | Token::LT | Token::LTE  => { op = Token::SUB; }
            _ => { op = op_tok; }
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
        let lib_content: String = self.libraries.get(&library).expect(&format!("No library exists with name: {}", library)).clone();
        // let lib_content = std::fs::read_to_string(lib_path).expect("could not read library");
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

        format!("{identifier}\n{args_out}{}ret\n", self.compile_node(block))
    }

    
    fn compile_if(&mut self, conditionals: Vec<Node>, alternative: Node) -> String {
        //Loop through conditionals, branch if condition to corresponding label (& postfix each with bra .endif)
        //Follow with <else> instructions followed with bra .endif
        let endif = self.generate_label("_l");

        let mut compiled_conditions = String::new();
        let mut compiled_consequences = String::new();
        for condition in conditionals {
            let condition_label = self.generate_label("_l");
            // let compiled_condition: String = String::new();

            if let Node::CONDITIONAL(condition_node, consequence) = condition {
                let branches: Vec<String> = self.get_conditional_branch(&condition_node);
                let condition_expr = self.compile_node(*condition_node);        

                let mut compiled_branches: String = String::new();
                for branch in branches {
                    compiled_branches += &format!("{branch} {condition_label}\n");
                }

                let compiled_consequence = self.compile_node(*consequence);

                compiled_conditions += &format!("{condition_expr}\n{compiled_branches}");
                compiled_consequences += &format!("{condition_label}\n{compiled_consequence}bra {endif}\n");
            }
        }

        let compiled_alternative = self.compile_node(alternative);
        format!("{compiled_conditions}{compiled_alternative}bra {endif}\n{compiled_consequences}{endif}\n")
    }


    fn compile_while(&mut self, condition_node: Node, consequence_node: Node) -> String {
        let beginwhile = self.generate_label("_l");
        let consequence = self.generate_label("_l");
        let endwhile = self.generate_label("_l");

        let branches: Vec<String> = self.get_conditional_branch(&condition_node);
        let compiled_conditional = self.compile_node(condition_node);        

        let mut compiled_branches: String = String::new();
        for branch in branches {
            compiled_branches += &format!("{branch} {consequence}\n");
        }

        let compiled_consequence = self.compile_node(consequence_node);

        format!("{beginwhile}\n{compiled_conditional}\n{compiled_branches}bra {endwhile}\n{consequence}\n{compiled_consequence}bra {beginwhile}\n{endwhile}\n")
    }


    fn compile_for(&mut self, declaration_node: Node, condition_node: Node, increment_node: Node, consequence_node: Node) -> String {
        let loop_label = self.generate_label("_l");
        let conseq_label = self.generate_label("_l");
        let endloop_label = self.generate_label("_l");

        let declaration = self.compile_node(declaration_node);

        let branch_instructions: Vec<String> = self.get_conditional_branch(&condition_node);
        let condition = self.compile_node(condition_node);        

        let mut branches: String = String::new();
        for branch in branch_instructions {
            branches += &format!("{branch} {conseq_label}\n");
        }

        let consequence = self.compile_node(consequence_node);
        let increment = self.compile_node(increment_node);

        format!("{declaration}{loop_label}\n{condition}\n{branches}bra {endloop_label}\n{conseq_label}\n{consequence}{increment}bra {loop_label}\n{endloop_label}\n")
    }


    fn get_conditional_branch(&self, infix: &Node) -> Vec<String> {
        match infix {
            Node::INFIX(_, op, _) => {
                match op {
                    Token::EE => { vec!["brz".to_string()] }
                    Token::NE => { vec!["bgt".to_string()] }

                    Token::LT => { vec!["blt".to_string()] }
                    Token::GT => { vec!["bgt".to_string()] }

                    Token::LTE => { vec!["blt".to_string(), "brz".to_string()] }
                    Token::GTE => { vec!["bgt".to_string(), "brz".to_string()] }

                    _ => { vec!["bgt".to_string()] }
                }
            }
            _ => { vec!["bgt".to_string()] }
        }
    }


    fn compile_return(&mut self, expression_node: Node) -> String {
        let expr_out = self.compile_node(Node::DECLARATION(
            "_ret".to_string(), 
            Box::new(expression_node)
        ));
        format!("{expr_out}ret\n")
    }


    fn generate_label(&mut self, id: &str) -> String {
        let label = id.to_owned() + &self.label_index.to_string();
        self.label_index += 1;
        label
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

        String::from("call _main\nhlt\nx dat 0\nlda _1\nsta x\n_1 dat 1\n_ret dat 0")
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