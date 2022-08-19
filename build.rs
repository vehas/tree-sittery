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

    let rust_dir: PathBuf = ["tree-sitter-rust", "src"].iter().collect();

    // println!("cargo:rerun-if-changed={}", rust_dir.join("parser.c").to_str().ok_or(""));
    // println!("cargo:-Wunused-parameter{}", source_file);

    cc::Build::new()
        .include(&rust_dir)
        .file(rust_dir.join("parser.c"))
        .file(rust_dir.join("scanner.c"))
        // .flag("-Wunused-parameter")
        // .flag("-Wunused-but-set-variable")
        .warnings(false)
        .compile("tree-sitter-rust");
}
