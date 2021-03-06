use serde_json::{self, Value};
use std::io::{self, prelude::*};
use std::process::exit;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        if let Err(e) = serde_json::from_str::<Value>(&line) {
            eprintln!("cjl: bad line: {}", e);
            eprintln!("cjl: the bad line is:");
            eprintln!("{}", line);
            exit(1);
        }
    }
    Ok(())
}
