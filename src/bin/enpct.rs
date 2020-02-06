use getopt::prelude::*;
use my::{program_main, util};
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
};

program_main!("en%");

fn usage_line() -> String {
    format!("Usage: {} [-hnq]", util::program_name("en%"))
}

fn print_usage() -> Result<i32, Box<dyn Error>> {
    println!("{}", usage_line());
    println!("  -h   display this help");
    println!("  -n   encode newlines");
    println!("  -q   use query string formatting (space => '+' instead of '%20')");

    Ok(0)
}

fn program() -> Result<i32, Box<dyn Error>> {
    let mut opts = Parser::new(&util::program_args(), "dehnq");

    let mut linewise = true;
    let mut query = false;
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => return print_usage(),
                Opt('n', None) => linewise = false,
                Opt('q', None) => query = true,
                _ => unreachable!(),
            },
        }
    }

    if linewise {
        for line in BufReader::new(io::stdin()).lines() {
            let bytes: Vec<u8> = encode(line.unwrap().as_bytes(), query)?;
            println!(
                "{}",
                String::from_utf8(bytes)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
            );
        }
    } else {
        let mut input: Vec<u8> = Vec::new();
        BufReader::new(io::stdin()).read_to_end(&mut input)?;

        let bytes: Vec<u8> = encode(&input, query)?;
        print!(
            "{}",
            String::from_utf8(bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
        );
    }

    Ok(0)
}

fn encode(bytes: &[u8], query: bool) -> io::Result<Vec<u8>> {
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

    Ok(output)
}
