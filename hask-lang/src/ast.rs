#[derive(Clone, Debug)]
pub enum Type {
    Uint32,
    Uint64,
    Int32,
    Int64,
    None,
    Ptr(Box<Type>)
}

#[derive(Clone, Debug)]
pub enum Node {
    Type(Type),
    Name(String),
    Int(String),
    Op(String, Box<Node>, Box<Node>),    
    Assign(String, Box<Node>, Box<Node>),
    Re(String, Box<Node>),
    Import(String),
    Func(String, Vec<(String, Node)>, Box<Node>, Box<Node>),
    FuncCall(String, Vec<(String)>),
    Block(Vec<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>),
    Return(Box<Node>),
}