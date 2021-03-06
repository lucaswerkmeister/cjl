use atty::Stream;
use ringbuffer::{typenum, GenericRingBuffer, RingBuffer};
use serde_json::{self, Value};
use std::io::{self, prelude::*};
use std::process::exit;

fn strip_crlf(mut line: String) -> String {
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    line
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut previous_lines = GenericRingBuffer::<_, typenum::U4>::new();
    loop {
        let mut line = String::new();
        let read_result = input.read_line(&mut line);
        match read_result {
            Ok(0) => return Ok(()),
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if let Err(e) = serde_json::from_str::<Value>(&line) {
            println!("cjl: bad line: {}", e);
            println!("cjl: the previous (good) lines were:");
            while let Some(previous_line) = previous_lines.dequeue() {
                println!("{}", strip_crlf(previous_line));
            }
            println!("cjl: the bad line is:");
            println!("{}", strip_crlf(line));
            if atty::isnt(Stream::Stdin) {
                let mut next_line = String::new();
                if let Ok(n) = input.read_line(&mut next_line) {
                    if n > 0 {
                        println!("cjl: the next line is:");
                        println!("{}", strip_crlf(next_line));
                    }
                }
            }
            exit(1);
        }
        previous_lines.push(line);
    }
}
