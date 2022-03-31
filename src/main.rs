use std::fs;
use std::env;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct ProgError(String);

impl fmt::Display for ProgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for ProgError {}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        return Err(Box::new(ProgError("Wrong number of argumets".to_string())));
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let mut out_str = String::new();

    let raw_str = fs::read_to_string(input_file)?;

    out_str.push_str("<snippets namespace=\"\" license=\"BSD\" filetypes=\"rust\" authors=\"Philip K. Gisslow\" name=\"Rust snippets\">\n");
    out_str.push_str("  <script></script>\n");

    let mut in_snippet = false;
    for l in raw_str.lines() {
        // FIXME Use regex for this instead...
        if !in_snippet && l.contains("snippet") {
            println!("{}", l);
        }
        //out_str.push_str(&raw_str[..]);
    }

    out_str.push_str("</snippets>");

    println!("{}", out_str);

    Ok(())
}
