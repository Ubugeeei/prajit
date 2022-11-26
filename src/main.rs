mod jit;
mod parse;

fn main() {
    let ast = parse::parser::Parser::new(String::from("1 * 2 + 3 * 4")).parse();
    println!("{:#?}", ast);
}
