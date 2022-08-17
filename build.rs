use std::path::PathBuf;

fn main() {
    let gql_dir: PathBuf = ["tree-sitter-graphql", "src"].iter().collect();

    cc::Build::new()
        .include(&gql_dir)
        .file(gql_dir.join("parser.c"))
        .compile("tree-sitter-graphql");
    
    let sql_dir: PathBuf = ["tree-sitter-sql", "src"].iter().collect();

    cc::Build::new()
        .include(&sql_dir)
        .file(sql_dir.join("parser.c"))
        .file(sql_dir.join("scanner.cc"))
        .compile("tree-sitter-sql");
}
