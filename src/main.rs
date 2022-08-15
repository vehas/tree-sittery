use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_graphql() -> Language; }

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parser() {
    let language = unsafe { tree_sitter_graphql() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let source_code = "type a{b: String}";
    let tree = parser.parse(source_code, None).unwrap();

    assert_eq!(tree.root_node().to_sexp(), "(source_file (module_declaration (module_header (module_keyword) (module_identifier (simple_identifier))) (module_nonansi_header (list_of_ports))))");
}