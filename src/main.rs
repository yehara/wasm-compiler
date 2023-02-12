extern crate core;

mod wasmc;
mod parser;
mod ast;

use std::env;
use std::process::exit;

use wasmc::compile;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        exit(-1);
    }

    compile(&args[1]);

}

