use cli::{emit_assembly::emit_assembly, repl::start_repl, run::run};

mod cli;
mod eval;
mod parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        println!("argument is not provided.");
        std::process::exit(1);
    }

    let is_interactive = args.iter().any(|it| it == "-i" || it == "--interactive");
    let is_emit_assembly = args.iter().any(|it| it == "-e" || it == "--emit-assembly");
    let is_jit_compile = args.iter().any(|it| it == "-j" || it == "--jit-compile");

    if is_interactive {
        start_repl(is_jit_compile);
    } else if is_emit_assembly {
        emit_assembly(args);
    } else {
        run(args);
    }
}
