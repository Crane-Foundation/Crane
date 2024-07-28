//create an enum for the node types
#[derive(Debug, Clone)]
pub enum NodeType {
    Number,
    String,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Comment,
    Unknown,
    Expression,
    UnaryExpression,
    FunctionCall,
    Err,
    Assignment,
    Reassignment
}
//create a node struct
#[derive(Debug, Clone)]
pub struct Node {
    node_type: NodeType,
    value: Option<String>,
    line: usize,
    children: Vec<Node>,
}
//create a tree struct
#[derive(Debug, Clone)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
impl Node {
    pub fn new(node_type: NodeType, value: Option<String>, line: usize) -> Self {
        Self {
            node_type,
            value,
            line,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}