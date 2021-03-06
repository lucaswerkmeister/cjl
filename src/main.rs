use ringbuffer::{typenum, GenericRingBuffer, RingBuffer};
use serde_json::{self, Value};
use std::io::{self, prelude::*};
use std::process::exit;

fn main() -> io::Result<()> {
    let mut previous_lines = GenericRingBuffer::<_, typenum::U4>::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        if let Err(e) = serde_json::from_str::<Value>(&line) {
            eprintln!("cjl: bad line: {}", e);
            eprintln!("cjl: the previous (good) lines were:");
            while let Some(previous_line) = previous_lines.dequeue() {
                eprintln!("{}", previous_line);
            }
            eprintln!("cjl: the bad line is:");
            eprintln!("{}", line);
            exit(1);
        }
        previous_lines.push(line);
    }
    Ok(())
}
