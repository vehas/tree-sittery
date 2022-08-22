use std::collections::HashMap;

use tree_sitter::{Language, Parser, Query, QueryCapture, QueryCursor};
use strfmt::strfmt;
use anyhow::Error;

extern "C" {
    fn tree_sitter_graphql() -> Language;
}
extern "C" {
    fn tree_sitter_sql() -> Language;
}
extern "C" {
    fn tree_sitter_rust() -> Language;
}
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_gql_parser() {
    let gql_lang = unsafe { tree_sitter_graphql() };
    let mut gql_parser = Parser::new();
    gql_parser.set_language(gql_lang).unwrap();

    let source_code = r#"type aaaa{b: int @directive(a: "{{{}"), c: int, d: float} type bbb{b: int, c: int}"#;
    let gql_tree = gql_parser.parse(source_code, None).unwrap();

    println!("{:?}", gql_tree.root_node().to_sexp());

    let query = Query::new(
        gql_lang,
        r#"
        (object_type_definition
           (name) @tname
           (fields_definition
                (
                    (field_definition
                        (name) @nn
                        (type) @t
                    )
                    (comma)?
                )*
           )
        )"#,
    );
    let mut cursor = QueryCursor::new();
    let matchs = cursor
        .matches(
            &(query.unwrap()),
            gql_tree.root_node(),
            source_code.as_bytes(),
        )
        .map(|m| (m.pattern_index, m.captures.to_owned()))
        .collect::<Vec<(usize, Vec<QueryCapture>)>>();
    for (_q, qc) in matchs {
        for q in qc {
            println!(
                "{:?} {:?} {:?} {:?}",
                q.node.to_sexp(),
                q.node.start_byte(),
                q.node.end_byte(),
                &source_code[q.node.start_byte()..q.node.end_byte()]
            );
        }
        println!("--------------------------------")
    }
}

struct Attr{
    name: String,
    value_type: String
}
struct  Table<'a>{
    name: String,
    attrs: &'a[Attr]
}
static graphql_head: &str = "type {name} {{ }}";
static graphql_body: &str = "{}: {},";

fn codegen(table: Table, lang: String) -> String{
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), table.name);
    // vars.insert("body".to_string(), table.attrs);

    let mut code = strfmt(graphql_head, &vars);
    // let mut body = String::new();
    // for attr in table.attrs.iter() {
    //     code.push(format!(graphql_body.into(), attr.name, attr.value_type))
    // }
    // format!(code, body)
    let code_str = code.expect("fmt error");
    println!("code : {}", code_str);

    code_str
}

#[test]
fn test_sql_parser() -> Result<(), Error>{
    let sql_lang = unsafe { tree_sitter_sql() };
    let mut sql_parser = Parser::new();
    sql_parser.set_language(sql_lang).unwrap();

    let source_code = "create table a(b int, c int); create table b(b int, c int);";
    let sql_tree = sql_parser.parse(source_code, None).unwrap();

    println!("{:?}", sql_tree.root_node().to_sexp());

    let query = Query::new(
        sql_lang,
        r#"
        (create_table_statement
            (identifier) @table_name
            (table_parameters
                (
                    (table_column
                        name: (identifier) @name
                        type: (type) @type
                    )+
                    _*
                )+
            )
        )"#,
    );
    let mut cursor = QueryCursor::new();
    let matchs = cursor
        .matches(
            &(query.unwrap()),
            sql_tree.root_node(),
            source_code.as_bytes(),
        )
        .map(|m| (m.pattern_index, m.captures.to_owned()))
        .collect::<Vec<(usize, Vec<QueryCapture>)>>();
    // let mut match_iter = matchs.iter();
    // let table_type = match_iter.next().ok_or("don't have any data");
    // println!("table_type: {:?}", table_type);
    for (_q, qc) in matchs.iter() {
        let mut qnext = qc.iter();
        let qn = qnext.next().expect("table should have name").node;
        let qname = qn;
        let mut table = Table {
            name: source_code[qname.start_byte()..qname.end_byte()].to_owned(),
            attrs: &vec![] } ;
        for q in qnext {
            println!(
                "{:?} {:?} {:?} {:?}",
                q.node.to_sexp(),
                q.node.start_byte(),
                q.node.end_byte(),
                &source_code[q.node.start_byte()..q.node.end_byte()]
            );
        }
        println!("code gen {}", codegen(table, "".to_string()));
        println!("--------------------------------")
    }

    Ok(())
}


#[test]
fn test_rust_parser() {
    let rust_lang = unsafe { tree_sitter_rust() };
    let mut rust_parser = Parser::new();
    rust_parser.set_language(rust_lang).unwrap();

    let source_code = "struct a{c: usize, b: usize}";
    let rust_tree = rust_parser.parse(source_code, None).unwrap();

    println!("{:?}", rust_tree.root_node().to_sexp());

    let query = Query::new(
        rust_lang,
        r#"
        (struct_item
            name: (type_identifier) @struct_name
            body: (field_declaration_list 
                        (
                            (field_declaration 
                                name: (field_identifier) @name
                                type: (primitive_type) @type
                            )
                            _*
                        )+
                  )
        )
        "#,
    );
    let mut cursor = QueryCursor::new();
    let matchs = cursor
        .matches(
            &(query.unwrap()),
            rust_tree.root_node(),
            source_code.as_bytes(),
        )
        .map(|m| (m.pattern_index, m.captures.to_owned()))
        .collect::<Vec<(usize, Vec<QueryCapture>)>>();
    for (_q, qc) in matchs {
        for q in qc {
            println!(
                "{:?} {:?} {:?} {:?}",
                q.node.to_sexp(),
                q.node.start_byte(),
                q.node.end_byte(),
                &source_code[q.node.start_byte()..q.node.end_byte()]
            );
        }
        println!("--------------------------------")
    }
}