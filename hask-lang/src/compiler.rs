use crate::ast::{Type, Node};

fn compile_type(input_type: &Type) -> String {
    let mut compiled_output = String::new();

    match input_type {
        Type::Uint32 => {
            compiled_output += "unsigned int";
        }
        Type::Uint64 => {
            compiled_output += "unsigned long";
        }
        Type::Int32 => {
            compiled_output += "signed int";
        }
        Type::Int64 => {
            compiled_output += "signed long";
        }
        Type::None => {
            compiled_output += "void";
        }
        Type::Ptr(x) => {
            compiled_output += &format!("{} *", compile_type(x));
        }
    };

    compiled_output
}

fn compile_node(node: &Node) -> String {
    let mut compiled_output = String::new();

    match node {
        Node::Int(x) => {
            compiled_output += &x;
        }
        Node::Name(x) => {
            compiled_output += &x;
        }
        Node::Op(op, a, b) => {
            compiled_output += &format!("{} {} {}", compile_node(a), op, compile_node(b));
        }
        Node::Type(x) => {
            compiled_output += &compile_type(x);
        }
        Node::Assign(name, input_type, val) => {
            compiled_output += &format!("{} {} = {};\n", compile_node(input_type), name, compile_node(val));
        }
        Node::Re(name, val) => {
            compiled_output += &format!("{} = {};\n", name, compile_node(val));
        }
        Node::Return(e) => {
            compiled_output += &format!("return {};", compile_node(e));
        }
        Node::Block(all) => {
            for i in all.iter() {
                compiled_output += &compile_node(i);
            }
        }
        Node::Import(module) => {
            compiled_output += &format!("#include \"{}\";\n", module);
        }
        Node::Func(name, args, ret, body) => {
            compiled_output += &format!("\n{} {} (", compile_node(ret), name);
              
            for (name, i) in args.iter() {
                compiled_output += &format!("{} {},", compile_node(i), name);
            }

            compiled_output += &format!(") {{\n{}\n}}\n", &compile_node(body));
        }
        Node::FuncCall(func_name, args) => {
            compiled_output += &format!("{}(", func_name);
            for argument in args.iter() {
                compiled_output += &format!("{},", argument);
            }
        }
        Node::If(e, body, other) => {
            compiled_output += &format!("if ({}) {{\n{}}}", compile_node(e), compile_node(body));
            match other {
                Some(x) => {
                    compiled_output += &format!(" else {{\n{}}}", compile_node(x));
                }
                None => { }
            }
        }
        Node::While(e, body) => {
            compiled_output += &format!("while ({}) {{\n{}}}\n", compile_node(e), compile_node(body));
        }
    }

    compiled_output
}

pub fn compile(ast: &Vec<Node>) -> String {
    let mut compiled_output = String::new();
    for item in ast.iter() {compiled_output += &compile_node(item);}
    compiled_output
}