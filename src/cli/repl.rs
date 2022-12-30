use std::io::Write;

use crate::{
    eval::{host::eval, jit::jit_compile},
    parse::parse,
};

pub fn start_repl(is_jit_compile: bool) {
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let result = std::io::stdin().read_line(&mut input);
        if let Err(_) = result {
            println!("failed to read line.");
            std::process::exit(1);
        }

        if &input == "exit\n" {
            println!("bye.");
            break;
        }

        if is_jit_compile {
            let result = jit_compile(input.clone());
            println!("{}\n", result);
            continue;
        } else {
            let res = parse(input.clone());
            match res {
                Ok(ast) => {
                    let result = eval(ast);
                    println!("{}\n", result);
                }
                Err(e) => {
                    println!(
                        "[\x1b[31mError\x1b[0m] {:?}: {}\ninput: \x1b[33m{}\x1b[0m",
                        e.kind(),
                        e,
                        input
                    );
                }
            }
        }
    }
}
