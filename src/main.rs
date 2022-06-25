use std::env;
use std::iter::Peekable;
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

    let mut iter = exp.chars().peekable();
    let num:i32 = strtoi(&mut iter);
    println!("(module");
    println!("  (func $main (result i32)");
    println!("   i32.const {})", num);
    println!("  (export \"main\" (func $main))");
    println!(")");

}

fn strtoi<L: Iterator<Item = char>>(iter: &mut Peekable<L>) -> i32 {
    let mut num = 0;
    while let Some(c) = iter.peek() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
            iter.next();
        } else {
            break;
        }
    }
    num
}