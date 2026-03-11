use std::{error::Error, result::Result};
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator, Tree};

pub fn parse(src: &str) -> Result<Tree, Box<dyn Error>> {
    let mut parser = Parser::new();

    parser.set_language(&tree_sitter_go::LANGUAGE.into())?;

    let tree = parser.parse(&src, None).unwrap();

    Ok(tree)
}

#[derive(Debug, Clone)]
pub struct DiscoveredFunction<'a> {
    pub name: String,
    pub declaration: Node<'a>,
}

pub fn discover_functions<'a>(
    tree: &'a Tree,
    src: &str,
) -> Result<im::Vector<DiscoveredFunction<'a>>, Box<dyn Error>> {
    let mut cursor = QueryCursor::new();
    let mut nv = im::Vector::new();

    let query = Query::new(
        &tree_sitter_go::LANGUAGE.into(),
        r#"
        [
          (function_declaration
            name: (identifier) @name
          ) @func
          (method_declaration
            name: (field_identifier) @name
          ) @func
        ]
        "#,
    )?;

    let mut matches = cursor.matches(&query, tree.root_node(), src.as_bytes());

    while let Some(m) = matches.next() {
        let mut func_node = None;
        let mut name_node = None;

        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];

            match capture_name {
                "func" => func_node = Some(capture.node),
                "name" => name_node = Some(capture.node),
                _ => {}
            }
        }

        if let (Some(func_node), Some(name_node)) = (func_node, name_node) {
            nv.push_back(DiscoveredFunction {
                name: name_node.utf8_text(src.as_bytes()).unwrap().to_string(),
                declaration: func_node,
            });
        }
    }

    Ok(nv)
}

pub fn get_func_params<'a>(func_node: Node<'a>, src: &'a str) -> Vec<&'a str> {
    let mut types = Vec::new();

    let Some(params_node) = func_node.child_by_field_name("parameters") else {
        return types;
    };

    let mut cursor = params_node.walk();

    for param in params_node.named_children(&mut cursor) {
        if let Some(type_node) = param.child_by_field_name("type") {
            if let Ok(type_str) = type_node.utf8_text(src.as_bytes()) {
                types.push(type_str);
            }
        }
    }

    types
}

fn print_tree(node: tree_sitter::Node, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}", indent, node.kind());

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(child, depth + 1);
    }
}
