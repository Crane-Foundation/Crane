#![allow(dead_code, unused_imports)]
mod tree;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::throw;
use ansi_term::Colour::{Red, Yellow};
use tree::Node;
use tree::NodeType;
use tree::Tree;
//create a parser struct that uses peekable iterator for the tokens
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
    pub tree: Tree,
}

//initialize the parser struct
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
            tree: Tree::new(),
        }
    }
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
impl Parser {
    // Parse the tokens
    pub fn parse(&mut self) {
        while let Some(token) = self.next() {
            let node = self.parse_token(token);
            self.tree.add_node(node);
        }
    }

    // Parse a token
    fn parse_token(&mut self, token: Token) -> Node {
        match token.token_type {
            TokenType::Number(ref f) => {
                if let Some(next_token) = self.peek() {
                    if let TokenType::Operator(_) = next_token.token_type {
                        return self.parse_expression(token);
                    }
                }
                Node::new(NodeType::Number, Some(f.to_string()), token.line)
            },
            TokenType::Str(s) => Node::new(NodeType::String, Some(s), token.line),
            TokenType::Identifier(s) => {
                //check if next token is an operator
                
                // This could be a function call, a variable, or a keyword
                // Check if the next token is a parenthesis
                if self.peek().unwrap().token_type == TokenType::LeftParen {
                    self.next();
                    // This is a function call
                    let mut node = Node::new(NodeType::FunctionCall, Some(s.clone()), token.line);
                    // Parse the arguments
                    let mut args = Vec::new();
                    let mut expect_comma = false;
                    while let Some(token) = self.next() {
                        if token.token_type == TokenType::RightParen {
                            break;
                        }
                        if expect_comma {
                            if token.token_type != TokenType::Comma {
                                throw!(format!("Expected a comma in: {}",Yellow.paint(s)), token.line);
                            }
                            expect_comma = false;
                            continue;
                        }
                        let arg = self.parse_token(token);
                        args.push(arg);
                        expect_comma = true;
                    }
                    // Add the arguments to the node
                    for arg in args {
                        node.add_child(arg);
                    }
                    node
                } else if self.peek().unwrap().token_type == TokenType::Operator("Eq".to_string()) {
                    // Consume the '=' operator
                    self.next();

                    // The left-hand side should be the identifier
                    let lhs = Node::new(NodeType::Identifier, Some(s), token.line);

                    // Parse the right-hand side expression
                    let nxt = self.next().unwrap();
                    let rhs = self.parse_expression(nxt);

                    // Create a reassignment node
                    let mut reassignment_node = Node::new(NodeType::Reassignment, None, token.line);
                    reassignment_node.add_child(lhs);
                    reassignment_node.add_child(rhs);

                    reassignment_node
                } else {
                    Node::new(NodeType::Identifier, Some(s), token.line)
                }
            },
            // If the token is an operator, create a node with the operator type, left child, and right child
            TokenType::Operator(op) => Node::new(NodeType::Operator, Some(op), token.line),
            TokenType::Keyword(kw) => {
                // If the keyword is a function definition
                if kw == "def" {
                    // The next token should be an identifier
                    let name_token = self.next().unwrap();
                    if let TokenType::Identifier(name) = name_token.token_type {
                        // The next token should be a left parenthesis
                        let left_paren = self.next().unwrap();
                        if let TokenType::LeftParen = left_paren.token_type {
                            // Parse the arguments
                            let mut args = Vec::new();
                            let mut expect_comma = false;
                            while let Some(token) = self.next() {
                                if token.token_type == TokenType::RightParen {
                                    break;
                                }
                                if expect_comma {
                                    if token.token_type != TokenType::Comma {
                                        throw!(format!("Expected a comma in: {} {}", Yellow.paint("fn"), Yellow.paint(kw)), token.line);
                                    }
                                    expect_comma = false;
                                    continue;
                                }
                                if token.token_type == TokenType::RightParen || token.token_type == TokenType::LeftParen {
                                    drop(token);
                                } else {
                                    let arg = self.parse_token(token);
                                    args.push(arg);
                                    expect_comma = true;
                                }
                            }
                            // The next token should be a left brace
                            let left_brace = self.next().unwrap();
                            if let TokenType::LeftBrace = left_brace.token_type {
                                // Parse the function body
                                let mut body = Node::new(NodeType::Block, None, left_brace.line);
                                let mut brace_count = 1;
                                while let Some(token) = self.next() {
                                    if token.token_type == TokenType::LeftBrace {
                                        brace_count += 1;
                                    } else if token.token_type == TokenType::RightBrace {
                                        brace_count -= 1;
                                        if brace_count == 0 {
                                            break;
                                        }
                                    }
                                    if token.token_type == TokenType::RightBrace || token.token_type == TokenType::LeftBrace {
                                        drop(token);
                                    } else {
                                        let node = self.parse_token(token);
                                        body.add_child(node);
                                    }
                                }
                                // Create a function node
                                let mut function_node = Node::new(NodeType::Function, Some(name), token.line);
                                // Add the arguments to the function node
                                for arg in args {
                                    function_node.add_child(arg);
                                }
                                // Add the body to the function node
                                function_node.add_child(body);
                                function_node
                            } else {
                                throw!(format!("Expected a left brace after: {} {}", Yellow.paint("fn"), Yellow.paint(kw)), left_brace.line);
                                Node::new(NodeType::Err, None, left_brace.line)
                            }
                        } else {
                            throw!(format!("Expected a left parenthesis after: {} {}", Yellow.paint("fn"), Yellow.paint(kw)), left_paren.line);
                            Node::new(NodeType::Err, None, left_paren.line)
                        }
                    } else {
                        throw!(format!("Expected an identifier after: {} {}", Yellow.paint("fn"), Yellow.paint(kw)), name_token.line);
                        Node::new(NodeType::Err, None, name_token.line)
                    }
                } else if kw == "if" {
                    // the next token should be a left parenthesis, if not throw an error
                    let left_paren = self.next().unwrap();
                    if let TokenType::LeftParen = left_paren.token_type {
                        //there will be a full conditional expression within () so parse it
                        let nx = self.next().unwrap();
                        let condition = self.parse_token(nx);
                        // the next token should be a right parenthesis, if not throw an error
                        let right_paren = self.next().unwrap();
                        if let TokenType::RightParen = right_paren.token_type {
                            //pass
                        } else {throw!(format!("Expected a right parenthesis after: {}", Yellow.paint(kw)), right_paren.line);
                        Node::new(NodeType::Err, None, right_paren.line);}
                        // the next token should be a left brace, if not throw an error
                        let left_brace = self.next().unwrap();
                        if let TokenType::LeftBrace = left_brace.token_type {
                            // Parse the body
                            let mut body = Node::new(NodeType::Block, None, left_brace.line);
                            let mut brace_count = 1;
                            while let Some(token) = self.next() {
                                if token.token_type == TokenType::LeftBrace {
                                    brace_count += 1;
                                } else if token.token_type == TokenType::RightBrace {
                                    brace_count -= 1;
                                    if brace_count == 0 {
                                        break;
                                    }
                                }
                                if token.token_type == TokenType::RightBrace || token.token_type == TokenType::LeftBrace {
                                    drop(token);
                                } else {
                                    let node = self.parse_token(token);
                                    body.add_child(node);
                                }
                            }
                            // Create an if node
                            let mut if_node = Node::new(NodeType::Conditional, Some(kw), token.line);
                            // Add the condition to the if node
                            if_node.add_child(condition);
                            // Add the body to the if node
                            if_node.add_child(body);
                            if_node
                        } else {
                            throw!(format!("Expected a left brace after: {}", Yellow.paint(kw)), left_brace.line);
                            Node::new(NodeType::Err, None, left_brace.line)
                        }
                    } else {
                        throw!(format!("Expected a left parenthesis after: {}", Yellow.paint(kw)), left_paren.line);
                        Node::new(NodeType::Err, None, left_paren.line)
                    }
                } else {
                    Node::new(NodeType::Keyword, Some(kw), token.line)
                }
            },
            _ => {println!("{:#?}", token);Node::new(NodeType::Err, None, token.line)},
        }
    }

    // Parse an expression
    fn parse_expression(&mut self, first_token: Token) -> Node {
        let mut output_queue: Vec<Node> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        // Helper function to get precedence
        fn get_precedence(op: &str) -> i32 {
            //match statement, include precendence of boolean operators and arithmetic operators, and comparison operators
            match op {
                "Or" => 1,
                "And" => 2,
                "Eq" | "Ne" => 3,
                "Lt" | "Gt" | "Le" | "Ge" => 4,
                "Add" | "Sub" => 5,
                "Mul" | "Div" => 6,
                _ => 0,
            }
        }

        // Collect all tokens for the expression
        let mut current_token = Some(first_token.clone());
        let mut open = 0;
        while let Some(token) = current_token {
            match &token.token_type {
                TokenType::Number(f) => {
                    output_queue.push(Node::new(NodeType::Number, Some(f.to_string()), token.line));
                }
                TokenType::Str(s) => {
                    output_queue.push(Node::new(NodeType::String, Some(s.clone()), token.line));
                }
                TokenType::Identifier(s) => {
                    output_queue.push(Node::new(NodeType::Identifier, Some(s.clone()), token.line));
                }
                TokenType::Operator(op) => {
                    while let Some(top_op) = operator_stack.last() {
                        if let TokenType::Operator(top_op_str) = &top_op.token_type {
                            if get_precedence(top_op_str) >= get_precedence(op) {
                                let top_token = operator_stack.pop().unwrap();
                                let mut node = Node::new(NodeType::Operator, Some(top_token.as_string()), top_token.line);
                                let right = output_queue.pop().unwrap();
                                let left = output_queue.pop().unwrap();
                                node.add_child(left);
                                node.add_child(right);
                                output_queue.push(node);
                            } else {
                                break;
                            }
                        }
                    }
                    operator_stack.push(token);
                }
                TokenType::LeftParen => {
                    // Recursively parse the expression inside the parentheses
                    let nx = self.next().unwrap();
                    let expr_node = self.parse_expression(nx);
                    output_queue.push(expr_node);
                    open += 1;
                }
                TokenType::RightParen => {
                    // Stop parsing the current expression
                    open -= 1;
                    break;
                }
                _ => {}
            }
            current_token = self.next();
        }

        // Pop all operators from the stack
        while let Some(top_token) = operator_stack.pop() {
            let mut node = Node::new(NodeType::Expression, Some(top_token.as_string()), top_token.line);
            let right = output_queue.pop().unwrap();
            let left = output_queue.pop().unwrap();
            node.add_child(left);
            node.add_child(right);
            output_queue.push(node);
        }
        // The final node should be the root of the expression tree
        output_queue.pop().unwrap()
    }
}

fn drop<T>(_x: T) {
    // This function is used to drop the value of x
    // so that the compiler does not throw a warning
    // about unused variables
}