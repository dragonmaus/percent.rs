use getopt::prelude::*;
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
};

program::main!("de%");

fn usage_line() -> String {
    format!("Usage: {} [-hn]", program::name("de%"))
}

fn print_usage() -> Result<i32, Box<dyn Error>> {
    println!("{}", usage_line());
    println!("  -h   display this help");
    println!("  -n   newlines are encoded in input");

    Ok(0)
}

fn program() -> Result<i32, Box<dyn Error>> {
    let mut opts = Parser::new(&program::args(), "hn");

    let mut linewise = true;
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => return print_usage(),
                Opt('n', None) => linewise = false,
                _ => unreachable!(),
            },
        }
    }

    if linewise {
        for line in BufReader::new(io::stdin()).lines() {
            let bytes: Vec<u8> = decode(line.unwrap().as_bytes())?;
            println!(
                "{}",
                String::from_utf8(bytes)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
            );
        }
    } else {
        let mut input: Vec<u8> = Vec::new();
        BufReader::new(io::stdin()).read_to_end(&mut input)?;

        let bytes: Vec<u8> = decode(&input)?;
        print!(
            "{}",
            String::from_utf8(bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?
        );
    }

    Ok(0)
}

fn decode(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();
    let mut bytes = bytes.iter();

    loop {
        match bytes.next() {
            None => break,
            Some(b) => match *b {
                b'%' => {
                    let mut new_byte: u8 = 0;

                    match bytes.next() {
                        None => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "unexpected end of input",
                            ));
                        }
                        Some(x) => {
                            let x = *x as char;
                            if x.is_ascii_hexdigit() {
                                let x = match x.to_digit(16) {
                                    None => {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            format!("invalid input -- {:?}", x),
                                        ));
                                    }
                                    Some(x) => x,
                                };
                                new_byte += (x * 0x10) as u8;
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("invalid input -- {:?}", x),
                                ));
                            }
                        }
                    }

                    match bytes.next() {
                        None => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "unexpected end of input",
                            ));
                        }
                        Some(x) => {
                            let x = *x as char;
                            if x.is_ascii_hexdigit() {
                                let x = match x.to_digit(16) {
                                    None => {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            format!("invalid input -- {:?}", x),
                                        ));
                                    }
                                    Some(x) => x,
                                };
                                new_byte += x as u8;
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("invalid input -- {:?}", x),
                                ));
                            }
                        }
                    }

                    output.push(new_byte);
                }
                b'+' => output.push(b' '),
                _ => output.push(*b),
            },
        }
    }

    Ok(output)
}
