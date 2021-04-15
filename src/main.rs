use std::env;
use std::fs::File;
use std::io::{SeekFrom, Seek, Write, Read};
use std::fs::OpenOptions;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!(":/");
        return;
    } else if args.len() < 5 {
        println!("A simple Rust program that fabricates a TFTP image out of a stock image.\n
                Usage: {} <in file> <out file> <skip> <zero>\n
                \t<in file>: path to input file\n
                \t<out file>: path to output file\n
                \t<skip>: number of bytes to skip before loading <in file>\n
                \t<zero>: number of zeros to write to <out file>\n", args[0]);
        return;
    }

    let zeros: usize = args[4].parse::<usize>().expect("Wrong value for zeros");
    let skip: u64 = args[3].parse::<u64>().expect("Wrong value for skip");

    let mut fin = File::open(&args[1]).expect("Failed to open infile");
    let mut fout = OpenOptions::new().write(true).create(true)
        .open(&args[2]).expect("Failed to open outfile");

    let fin_size: u64 = fin.metadata().unwrap().len();
    if fin_size < skip {
        println!("Size of infile is smaller than skip value");
        return;
    }

    fin.seek(SeekFrom::Start(skip)).expect("Failed to skip");

    let load_size = (fin_size - skip) as usize;
    let mut buffer: Vec<u8> = vec![0; load_size];
    let zeros_buf: Vec<u8> = vec![0; zeros];

    fin.read(&mut buffer[..]).expect("Failed to load infile");

    fout.write(&zeros_buf).expect("Failed to write outfile");
    fout.write(&buffer).expect("Failed to write outfile");

    println!("Success");

    return;
}
