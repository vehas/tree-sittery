use std::collections::HashMap;

use phf::{phf_map};
use anyhow::Error;
use itertools::Itertools;
use std::cell::RefCell;
use strfmt::strfmt;
use tree_sitter::{Language, Parser, Query, QueryCapture, QueryCursor};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}
extern "C" {
    fn tree_sitter_sql() -> Language;
}
extern "C" {
    fn tree_sitter_rust() -> Language;
}
fn _main() {
    println!("Hello, world!");
}

#[test]
fn test_gql_parser() {
    let gql_lang = unsafe { tree_sitter_graphql() };
    let mut gql_parser = Parser::new();
    gql_parser.set_language(gql_lang).unwrap();

    let source_code =
        r#"type aaaa{b: int @directive(a: "{{{}"), c: int, d: float} type bbb{b: int, c: int}"#;
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

pub struct Attr {
    name: String,
    value_type: String,
}
pub struct Table {
    name: String,
    attrs: RefCell<Vec<Attr>>,
}


static GRAPHQL_HEAD: &str = "type {name} {{{{\n{{body}}\n }}}}";
static GRAPHQL_BODY: &str = "    {name}: {type}, \n";

static TYPE_GRAPHQL: phf::Map<&'static str, &'static str> = phf_map! {
    "int" => "Int",
    "text" => "String",
    "varchar" => "String",

    "usize" => "Int"
};

pub fn codegen(table: Table, _lang: String) -> String {
    let type_graphql = &TYPE_GRAPHQL;

    let mut vars = HashMap::new();
    vars.insert("name".to_string(), table.name);

    let code = strfmt(GRAPHQL_HEAD, &vars);
    let mut body = String::new();
    for attr in table.attrs.borrow().iter() {
        let mut varsa = HashMap::new();
        varsa.insert("name".to_string(), attr.name.clone());
        varsa.insert(
            "type".to_string(),
            type_graphql
                .get(&attr.value_type)
                // .ok_or(attr.value_type.clone())
                .unwrap_or(&attr.value_type.as_ref())
                .to_string()
        );
        let bc = strfmt(GRAPHQL_BODY, &varsa);
        let bcs = bc.expect("body fmt error");
        // println!("bcs {:?}", bcs);
        body.push_str(&bcs);
    }
    let code_str = code.expect("fmt error");
    // println!("code : {}", code_str);
    let mut varb = HashMap::new();
    varb.insert("body".to_string(), body);
    let full_table = strfmt(&code_str, &varb).expect("full table format error");
    // print!("full table {:?}", full_table);

    full_table
}

fn parse_then_gen_code(parse_gen: ParseGen) -> Result<(), Error> {
    let lang = parse_gen.lang;
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();

    let source_code = &parse_gen.code;
    let tree = parser.parse(source_code, None).unwrap();

    println!("AST of {}: {:?}", parse_gen.from_lang, tree.root_node().to_sexp());

    let query = Query::new(
        lang,
        &parse_gen.query,
    );
    let mut cursor = QueryCursor::new();
    let matchs = cursor
        .matches(
            &(query.unwrap()),
            tree.root_node(),
            source_code.as_bytes(),
        )
        .map(|m| (m.pattern_index, m.captures.to_owned()))
        .collect::<Vec<(usize, Vec<QueryCapture>)>>();
    for (_q, qc) in matchs.iter() {
        let mut qnext = qc.iter();
        let qn = qnext.next().expect("table should have name").node;
        let qname = qn;
        let table = Table {
            name: source_code[qname.start_byte()..qname.end_byte()].to_owned(),
            attrs: RefCell::new(Vec::new()),
        };
        for nn in &qnext.chunks(2) {
            let mut ni = nn.into_iter();
            let name_node = ni.next().expect("");
            let type_node = ni.next().expect("");
            // println!(
            //     "attr : {:?} {:?}",
            //     &source_code[name_node.node.start_byte()..name_node.node.end_byte()],
            //     &source_code[type_node.node.start_byte()..type_node.node.end_byte()]
            // );
            table.attrs.borrow_mut().push(Attr {
                name: source_code[name_node.node.start_byte()..name_node.node.end_byte()]
                    .to_string(),
                value_type: source_code[type_node.node.start_byte()..type_node.node.end_byte()]
                    .to_string(),
            });
        }
        println!("origin (all table)-----");
        println!("{}", source_code);
        println!("result of one table to lang {} -----", parse_gen.to_lang);
        println!("{}", codegen(table, "".to_string()));
        println!("--------------------------------")
    }

    Ok(())
}

static SQL_SOURCE_CODE: &str =
    r#"create table student(
        age int,
        height int,
        friend_count int);
      create table b(b int, c int);"#;
static SQL_QUERY: &str = 
    r#"
    (create_table 
        (_)*
        (table_reference 
            name: (identifier) @name
        ) 
        (column_definitions 
            (
                (column_definition 
                    name: (identifier) @attr_name
                    type: (_) @attr_type
                )
                _*
            )+
        )
    )
    "#;


struct ParseGen {
    from_lang: String,
    lang: Language,
    code: String,
    query: String,

    to_lang: String,
}
#[test]
fn test_sql_to_graphql() -> Result<(), Error>{
    let pg = ParseGen{
        from_lang: "sql".to_string(),
        lang: unsafe { tree_sitter_sql() },
        code: SQL_SOURCE_CODE.to_string(),
        query: SQL_QUERY.to_string(),
        to_lang: "graphql".to_string()};
    parse_then_gen_code(pg)
}

#[test]
fn test_rust_to_graphql() -> Result<(), Error>{
    let parse_gen = ParseGen{
        from_lang: "rust".to_string(),
        lang: unsafe { tree_sitter_rust() },
        code: "struct a{c: usize, b: usize}".to_string(),
        query: r#"
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
        "#.to_string(),
        to_lang: "graphql".to_string() };
    parse_then_gen_code(parse_gen)
}

