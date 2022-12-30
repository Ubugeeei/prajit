use cli::{repl::start_repl, run::run};

mod cli;
mod eval;
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
        start_repl();
    } else {
        run(args);
    }
}
