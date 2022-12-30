use crate::{eval::compile::compile, parse::parse};

pub fn emit_assembly(args: Vec<String>) {
    args[1..]
        .into_iter()
        .filter(|x| x != &"-i" && x != &"--interactive" && x != &"-e" && x != &"--emit-assembly")
        .for_each(|input| {
            let res = parse(String::from(input));
            match res {
                Ok(ast) => {
                    print!("{}", compile(ast));
                }
                Err(e) => {
                    println!(
                        "\n[\x1b[31mError\x1b[0m] {:?}: {}\ninput: \x1b[33m{}\x1b[0m",
                        e.kind(),
                        e,
                        input
                    );
                    std::process::exit(1);
                }
            }
        });
}
