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
        .warnings(false)
        .compile("tree-sitter-sql");

    let rust_dir: PathBuf = ["tree-sitter-rust", "src"].iter().collect();
    cc::Build::new()
        .include(&rust_dir)
        .file(rust_dir.join("parser.c"))
        .file(rust_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-rust");

}
