mod tree;
use crate::lexer::Token;
use crate::lexer::TokenType;
use tree::Node;
use tree::NodeType;
use tree::Tree;
use ansi_term::Colour::Red;
use crate::throw;
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
//create functions for parsing
impl Parser {
    //parse the tokens
    pub fn parse(&mut self) {
        while let Some(token) = self.next() {
            let node = self.parse_token(token);
            self.tree.add_node(node);
        }
    }
    //parse a token
    fn parse_token(&mut self, token: Token) -> Node {
        match token.token_type {
            TokenType::Number(f) => {
                let node = Node::new(NodeType::Number, Some(f), token.line);
                node
            }
            TokenType::Str(s) => {
                let node = Node::new(NodeType::String, Some(s), token.line);
                node
            }
            TokenType::Identifier(s) => {
                //this could be a function call, a variable, or a keyword
                //check if the next token is a parenthesis
                if (*self).peek().unwrap().token_type == TokenType::LeftParen {
                    //this is a function call
                    let mut node = Node::new(NodeType::Identifier, Some(s), token.line);
                    //parse the arguments
                    let mut args = Vec::new();
                    while let Some(token) = self.next() {
                        if token.token_type == TokenType::RightParen {
                            break;
                        }
                        let arg = self.parse_token(token);
                        args.push(arg);
                    }
                    //add the arguments to the node
                    for arg in args {
                        node.add_child(arg);
                    }
                    node
                } else {
                    //this is a variable or a keyword
                    let node = Node::new(NodeType::Identifier, Some(s), token.line);
                    node
                }
            }
            _ => {
                let node = Node::new(NodeType::Err, None, token.line);
                node
            }
        }
    }
}