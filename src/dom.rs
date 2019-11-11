use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

enum NodeType {
    Text(String),
    Comment(String),
    Element(ElementData)
}

type AttrMap = HashMap<String, String>;
struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data)
    }
}

fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data)
    }
}

fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element( ElementData {
            tag_name: name,
            attributes: attrs
        })
    }
}

fn print(root: Node) {
    let prefix = String::from("");
    _print(root, &prefix);
}

fn _print(root: Node, prefix: &String) {
    match root.node_type {
        NodeType::Comment(_) => {},
        NodeType::Text(data) => println!("{}{}", prefix, data),
        NodeType::Element(data) => println!("{}{}", prefix, data.tag_name)
    }

    let new_prefix = " ".repeat(prefix.len()) + " |--";
    for child in root.children {
        _print(child, &new_prefix);
    }
}
