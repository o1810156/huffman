use huffman::huffman::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let w_filename = args.next();
    if w_filename.is_none() {
        return Err(Error::new(ErrorKind::NotFound, "Please Pass Filename."));
    }

    let filename = w_filename.unwrap();
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let list = contents
        .split('\n')
        .map(|line| {
            let mut t = line.split(',');
            let k = t.next().unwrap_or("");
            let v = t.next().unwrap_or("0");
            (k, v.parse::<f64>().unwrap_or(0f64))
        })
        .collect::<Vec<_>>();

    let entropy = list.iter().map(|(_, v)| v * (-v.log2())).sum::<f64>();
    let huf_code = HuffmanCode::new(list).unwrap();

    println!(
        "entropy: {}\n{}\n{}\navg_len: {}",
        entropy,
        huf_code.get_tree(),
        huf_code,
        huf_code.get_avg_len()
    );

    Ok(())
}
