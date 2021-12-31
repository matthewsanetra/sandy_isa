fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .generate_in_source_tree()
        .process_file("src/parser/grammar.lalrpop")
        .unwrap();
}
