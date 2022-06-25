use std::env;
use std::process::exit;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        exit(-1);
    }

    compile(&args[1]);

}

fn compile(exp: &str) {

    let num:i32 = exp.parse().unwrap();
    println!("(module");
    println!("  (func $main (result i32)");
    println!("   i32.const {})", num);
    println!("  (export \"main\" (func $main))");
    println!(")");

}