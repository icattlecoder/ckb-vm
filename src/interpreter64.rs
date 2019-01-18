use ckb_vm::{run, SparseMemory};
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut file = File::open(args[0].clone()).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let args: Vec<Vec<u8>> = args.into_iter().map(|a| a.into_bytes()).collect();

    let start = Instant::now();
    let result = run::<u64, SparseMemory>(&buffer, &args);
    let end = Instant::now();

    println!("Result: {:?}", result);
    println!("Running time: {:?}", end.duration_since(start));

    exit(i32::from(result.unwrap_or(255)));
}
