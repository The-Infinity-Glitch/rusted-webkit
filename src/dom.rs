use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttributeMap,
}

type AttributeMap = HashMap<String, String>;

fn new_text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn new_element(tag_name: String, attributes: AttributeMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name,
            attributes,
        }),
    }
}
