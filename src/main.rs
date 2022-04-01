use std::fs;
use std::env;
use std::error::Error;
use std::fmt;
use regex::Regex;
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

    let re = Regex::new(r"(\S*)\s(\S*)\s(.*)").unwrap();
    let mut trigger_vec = Vec::new();
    let mut desc_vec = Vec::new();
    let mut templ_vec = Vec::new();
    let mut templ_str = String::new();

    let mut iter = raw_str.lines();
    while let Some(l) = iter.next() {
        if l.contains("snippet ") {
            if let Some(caps) = re.captures(&l[..]) {
                if let Some(g) = caps.get(2) {
                    trigger_vec.push(String::from(g.as_str()));
                    println!("=== {} ===", g.as_str());
                }
                if let Some(g) = caps.get(3) {
                    desc_vec.push(g.as_str().replace("\"", ""));
                }
            }
            templ_str.clear();
            let mut it_clone = iter.clone();
            while let Some(l) = it_clone.next() {
                if !l.starts_with("snippet "){
                    if !l.starts_with("# ") {
                        templ_str.push_str(&l[..]);
                        templ_str.push_str("\n");
                    }
                }
                else {
                    println!("{}", l);
                    println!("{}", templ_str);
                    templ_vec.push(templ_str.clone());
                    println!("Push");
                    break;
                }
            }
        }
    }

    println!("{}", templ_str);
    templ_vec.push(templ_str.clone());

    println!("{}", templ_vec.len());
    println!("{}", trigger_vec.len());
    assert!(templ_vec.len() == trigger_vec.len());

    for i in 0..templ_vec.len() {
        out_str.push_str("  <item>\n");
        out_str.push_str(std::format!("    <match>{}</match>\n", &trigger_vec[i]).as_str());
        out_str.push_str(std::format!("    <fillin>{}\n    </fillin>\n", &templ_vec[i]).as_str());
        out_str.push_str("  </item>\n");
    }

    out_str.push_str("</snippets>");

    println!("{}", out_str);

    Ok(())
}
