use super::{super::parse::parse, compile::compile};

pub fn jit_compile(input: String) -> i32 {
    let ast = parse(input).unwrap();

    let asm = compile(ast);

    let _ = std::fs::create_dir("prajit_out");
    let _ = std::fs::write("prajit_out/out.s", asm);

    let _ = std::process::Command::new("cc")
        .arg("prajit_out/out.s")
        .arg("-o")
        .arg("prajit_out/a.out")
        .output();

    let out = std::process::Command::new("./prajit_out/a.out")
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all("prajit_out");

    out.status.code().unwrap() as i32
}
