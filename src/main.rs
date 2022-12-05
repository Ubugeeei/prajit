use std::io::Write;

mod eval;
mod jit;
mod parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 0 {
        println!("argument is not provided.");
        std::process::exit(1);
    }

    let is_interactive = match args.iter().find(|it| it == &"-i" || it == &"--interactive") {
        Some(_) => true,
        None => false,
    };

    if is_interactive {
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

            let ast = parse::parser::Parser::new(input).parse();
            let result = eval::eval(ast);
            println!("{}", result);
        }
    } else {
        args[1..].into_iter().for_each(|input| {
            let ast = parse::parser::Parser::new(String::from(input)).parse();
            let result = eval::eval(ast);
            print!("{} ", result);
        });
    }
}
