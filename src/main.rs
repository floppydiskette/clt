use std::io::{Read, Write};

fn usage() {
    println!("clt: compress lossy text");
    println!("usage: clt [options] [file] [output]");
    println!("if no file is specified, stdin is used");
    println!("if no output is specified, stdout is used");
    println!("options:");
    println!("  -h, --help: show this help message");
    println!("  -v, --version: show version");
    println!("  -c, --compress: compress file (default)");
    println!("  -d, --decompress: decompress file");
    println!("  -[1-5]: compression level (default: 3)");
    println!("  -[1-4]: decompression level (default: 3)");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }
    let mut compress = true;
    let mut level = 3;
    let mut file = None;
    let mut out_file = None;
    for arg in args.iter().skip(1) {
        if arg == "-h" || arg == "--help" {
            usage();
            return;
        } else if arg == "-v" || arg == "--version" {
            println!("clt {}", env!("CARGO_PKG_VERSION"));
            return;
        } else if arg == "-c" || arg == "--compress" {
            compress = true;
        } else if arg == "-d" || arg == "--decompress" {
            compress = false;
        } else if arg.starts_with("-") {
            let num = arg[1..].parse::<u8>();
            if num.is_err() {
                println!("invalid option: {}", arg);
                return;
            }
            let num = num.unwrap();
            if compress {
                if num < 1 || num > 5 {
                    println!("invalid compression level: {}", num);
                    return;
                }
                level = num;
            } else {
                if num < 1 || num > 4 {
                    println!("invalid decompression level: {}", num);
                    return;
                }
                level = num;
            }
        } else if file.is_none() {
            file = Some(arg);
        } else if out_file.is_none() {
            out_file = Some(arg);
        } else {
            println!("too many arguments");
            return;
        }
    }

    let input = {
        if let Some(file) = file {
            let file = std::fs::File::open(file);
            if file.is_err() {
                println!("failed to open file: {}", file.unwrap_err());
                return;
            }
            let mut string = String::new();
            file.expect("failed to open file").read_to_string(&mut string).expect("failed to read file");
            string
        } else {
            let mut string = String::new();
            std::io::stdin().read_to_string(&mut string).expect("failed to read stdin");
            string
        }
    };

    let out = if compress {
        let compressor = match level {
            1 => lossy_text_compression::COMPRESSOR_LEVEL_1,
            2 => lossy_text_compression::COMPRESSOR_LEVEL_2,
            3 => lossy_text_compression::COMPRESSOR_LEVEL_3,
            4 => lossy_text_compression::COMPRESSOR_LEVEL_4,
            5 => lossy_text_compression::COMPRESSOR_LEVEL_5,
            _ => unreachable!(),
        };
        compressor.compress(&input)
    } else {
        let decompressor = match level {
            1 => lossy_text_compression::DECOMPRESSOR_LEVEL_1,
            2 => lossy_text_compression::DECOMPRESSOR_LEVEL_2,
            3 => lossy_text_compression::DECOMPRESSOR_LEVEL_3,
            4 => lossy_text_compression::DECOMPRESSOR_LEVEL_4,
            _ => unreachable!(),
        };
        decompressor.decompress(&input)
    };

    if let Some(file) = out_file {
        let file = std::fs::File::create(file);
        if file.is_err() {
            println!("failed to create file: {}", file.unwrap_err());
            return;
        }
        file.expect("failed to create file").write_all(out.as_bytes()).expect("failed to write file");
    } else {
        std::io::stdout().write_all(out.as_bytes()).expect("failed to write stdout");
    }
}