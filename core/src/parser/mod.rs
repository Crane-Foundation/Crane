#![allow(dead_code, unused_imports)]
mod tree;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::throw;
use ansi_term::Colour::Red;
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
                // This could be a function call, a variable, or a keyword
                // Check if the next token is a parenthesis
                if self.peek().unwrap().token_type == TokenType::LeftParen {
                    self.next();
                    // This is a function call
                    let mut node = Node::new(NodeType::FunctionCall, Some(s), token.line);
                    // Parse the arguments
                    let mut args = Vec::new();
                    while let Some(token) = self.next() {
                        if token.token_type == TokenType::RightParen {
                            break;
                        }
                        let arg = self.parse_token(token);
                        args.push(arg);
                    }
                    // Add the arguments to the node
                    for arg in args {
                        node.add_child(arg);
                    }
                    node
                } else if self.peek().unwrap().token_type == TokenType::Operator("=".to_string()) {
                    // Consume the '=' operator
                    self.next();

                    // The left-hand side should be the identifier
                    let lhs = Node::new(NodeType::Identifier, Some(s), token.line);

                    // Parse the right-hand side expression
                    let rhs = self.parse_expression(self.next().unwrap());

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
            TokenType::Keyword(kw) => Node::new(NodeType::Keyword, Some(kw), token.line),
            _ => Node::new(NodeType::Err, None, token.line),
        }
    }

    // Parse an expression
    fn parse_expression(&mut self, first_token: Token) -> Node {
        let mut output_queue: Vec<Node> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        // Helper function to get precedence
        fn get_precedence(op: &str) -> i32 {
            match op {
                "Add" | "Sub" => 1,
                "Mul" | "Div" => 2,
                _ => 0,
            }
        }

        // Collect all tokens for the expression
        let mut current_token = Some(first_token);
        while let Some(token) = current_token {
            match &token.token_type {
                TokenType::Number(f) => {
                    output_queue.push(Node::new(NodeType::Number, Some(f.to_string()), token.line));
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
                }
                TokenType::RightParen => {
                    // Stop parsing the current expression
                    break;
                }
                _ => {}
            }
            current_token = self.next();
        }

        // Pop all operators from the stack
        while let Some(top_token) = operator_stack.pop() {
            let mut node = Node::new(NodeType::Operator, Some(top_token.as_string()), top_token.line);
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
