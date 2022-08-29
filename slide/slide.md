---
marp: true
theme: gaia
class:
  - lead
  - invert
---

# incremental parsing with tree-sitter

veha suwat
github.com/vehas
fb.com/vehas

---

# why ?

---

# what is parser
#### program that given structured text, formal grammar turn data structure
## text + grammar -> data structure
---

text          : 1 * 2 + 3
grammar       : root -> num (ops num)+ 
data structure: 
![bg right:40% 80%](ds-node.svg)

[comment]: # (graph TD           )
[comment]: # (   R[Root] --> A[1])
[comment]: # (       R --> B[*]  )
[comment]: # (       B --> C[2]  )
[comment]: # (       R --> D[+]  )
[comment]: # (       D --> E[3]  )

---
# incremental parser (tree sitter)
#### parse faster the next time you parse almost the same thing
  - realtime interactive text editor
    - neovim (https://neovim.io/doc/treesitter/)
    - helix
    - code mirror (lezer)
  - github (small change in commit)
    - code diff
    - syntax highlight
    - "go to ..." in the web
    - vulnerability check
---

# speed

Speed - When initially parsing a file, tree-sitter-rust takes around twice as long as Rustc's hand-coded parser.
```
$ wc -l examples/ast.rs
  2157 examples/ast.rs

$ rustc -> (7 ms)

$ tree-sitter -> 16 ms then -> 1 ms
```

---

# use case
## for (incremental) parser
- Complex text manipulation
  - syntax highlighting
  - code formatter
  - linting rules
  - code search
  - vulnerability check
  - code visualization
  - data pipeline
  - rust 
  macro



"type {name} {{{{\n{{body}}\n }}}}";
---

# use parser
text + grammar -> AST
AST + tree query -> useful data
AST + visitor pattern -> useful AST

---
## project

## sql -> graphql
## rust -> graphql
---
# use parser
## build from C
```rust
    let gql_dir: PathBuf = ["tree-sitter-graphql", "src"].iter().collect();
    cc::Build::new()
        .include(&gql_dir)
        .file(gql_dir.join("parser.c"))
        .compile("tree-sitter-graphql");
```

---

# AST example
```sql
create table student(
        age int,
        height int,
        friend_count int);
```
---
# AST example
```lisp
(program
  (statement
    (create_table
      (keyword_create) (keyword_table)
      (table_reference
        name: (identifier))
        (column_definitions
          (column_definition
            name: (identifier)
            type: (keyword_int))
          (column_definition
            name: (identifier)
            type: (keyword_int))
          (column_definition name: (identifier) type: (keyword_int))))))
```
---

# query
```lisp
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
```
---

query result

name -> student
attr_name -> age, height
attr_type -> int, int

---

# code generator
```rust
strfmt(
static GRAPHQL_HEAD: &str = strfmt("type {name} {{{{\n{{body}}\n }}}}", table_name);
static GRAPHQL_BODY: &str = strfmt("    {name}: {type}, \n", body));
```
---

# make a new parser

---

# make a new parser 
### mermaid.js
### example
```mermaid.js
erDiagram
  student {
    int age
    int height
  }
  student }|..|{ classroom : in
```
![bg right:40% 80%](er.svg)

---
# mermaid spec

https://github.com/mermaid-js/mermaid/blob/develop/src/diagrams/er/parser/erDiagram.jison 

---

# show mermaid result

---

# test first

---

## fun

---

# other rust parser
- https://www.shadaj.me/writing/introducing-rust-sitter/
- https://pest.rs/

---

# thank you
## Q & A