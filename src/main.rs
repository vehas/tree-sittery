use tree_sitter::{Language, Parser, Query, QueryCapture, QueryCursor};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}
extern "C" {
    fn tree_sitter_sql() -> Language;
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_gql_parser() {
    let gql_lang = unsafe { tree_sitter_graphql() };
    let mut gql_parser = Parser::new();
    gql_parser.set_language(gql_lang).unwrap();

    let source_code = "type aaaa{b: int, c: int} type bbb{b: int, c: int}";
    let gql_tree = gql_parser.parse(source_code, None).unwrap();

    println!("{:?}", gql_tree.root_node().to_sexp());
    // (field_definition
    // (name) @nn
    // (type) @t))
    //         (object_type_definition
    // (name) @otd
    // (field_definition
    //     (field_definition
    //         (name) @nn
    //         (type) @t)*)))
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

#[test]
fn test_sql_parser() {
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
                (table_column
                    name: (identifier) @name
                    type: (type) @type
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
