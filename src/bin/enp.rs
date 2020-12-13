use getopt::prelude::*;
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
};

program::main!("en%");

fn usage_line() -> String {
    format!("Usage: {} [-h] [-nq]", program::name("en%"))
}

fn print_usage() {
    println!("{}", usage_line());
    println!("  -n   encode newlines");
    println!("  -q   use query string formatting (space => '+' instead of '%20')");
    println!();
    println!("  -h   display this help");
}

fn program() -> Result<i32, Box<dyn Error>> {
    let mut opts = Parser::new(&program::args(), "dehnq");

    let mut linewise = true;
    let mut query = false;
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('n', None) => linewise = false,
                Opt('q', None) => query = true,
                Opt('h', None) => {
                    print_usage();
                    return Ok(0);
                }
                _ => unreachable!(),
            },
        }
    }

    if linewise {
        for line in BufReader::new(io::stdin()).lines() {
            let bytes: Vec<u8> = encode(line.unwrap().as_bytes(), query);
            println!(
                "{}",
                String::from_utf8(bytes)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
            );
        }
    } else {
        let mut input: Vec<u8> = Vec::new();
        BufReader::new(io::stdin()).read_to_end(&mut input)?;

        let bytes: Vec<u8> = encode(&input, query);
        print!(
            "{}",
            String::from_utf8(bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
        );
    }

    Ok(0)
}

fn encode(bytes: &[u8], query: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for byte in bytes {
        for b in match *byte {
            b'-' | b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z' | b'~' => vec![*byte],
            b' ' if query => vec![b'+'],
            _ => format!("%{:02X}", byte).into_bytes(),
        } {
            output.push(b);
        }
    }

    output
}
