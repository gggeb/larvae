use std::env;
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::Path;

const STYLE: &str = "style.css";
const EXTRAS_DIR: &str = "ext";
const INPUT_DIR: &str = "./pages";
const OUTPUT_DIR: &str = "./output";

#[derive(PartialEq)]
enum Pen {
    Output,
    Kind,
    Args,
}

struct Tag {
    kind: String,
    args: String,
}

impl Tag {
    fn new() -> Self {
        Self {
            kind: String::new(),
            args: String::new(),
        }
    }

    fn pop_args(&mut self) { self.args.pop(); }

    fn push_kind(&mut self, c: char) { self.kind.push(c); }
    fn push_args(&mut self, c: char) { self.args.push(c); }

    fn construct(&self) -> (String, String) {
        let trimmed = self.kind.trim();
        let str = trimmed;
        if str == "HEADING" {
            let size = if &self.args == "" { "1".to_string() } else { self.args.clone() };

            (format!("<h{}>", size), format!("</h{}>", size))
        } else if str == "SUBTITLE" {
            ("<div class='subtitle'>".to_string(), "</div>".to_string())
        } else if str == "LINK" {
            (format!("<a href='{}'>", &self.args).to_string(), "</a>".to_string())
        } else if str == "E" {
            let class = self.args.clone().chars().map(|x| {
                match x {
                    'B' => { "bold" }
                    'I' => { "italic" }
                    'U' => { "underlined" }
                    'S' => { "secondary" }
                    _ => ""
                }
            }).fold(String::new(), |acc, x| {
                if acc.len() > 0 {
                    format!("{} {}", acc, x)
                } else {
                    x.to_string()
                }
            });

            (format!("<span class='{}'>", class).to_string(), "</span>".to_string())
        } else if str == "ALIGN" {
            let align = self.args.trim().to_lowercase();

            (format!("<div style='text-align: {}'>", align), "</div>".to_string())
        } else {
            ("".to_string(), "".to_string())
        }
    }
}

fn parse(str: String) -> String {
    let mut output = String::new();
    let mut stack: Vec<Tag> = Vec::new();
    let mut pen = Pen::Output;
    let mut prev_char = ' ';

    for c in str.chars() {
        match c {
            '[' => {
                if prev_char != '\\' {
                    pen = Pen::Kind;
                    stack.push(Tag::new());

                    continue
                }
            }
            ';' => {
                if pen == Pen::Kind {
                    pen = Pen::Args;

                    continue
                }
            }
            ':' => {
                if pen == Pen::Kind || pen == Pen::Args {
                    if prev_char == '\\' && pen == Pen::Args {
                        stack.last_mut().unwrap().pop_args();
                    } else {
                        let (open_tag, _) = stack.last().unwrap().construct();
                        output.push_str(&open_tag);

                        pen = Pen::Output;

                        continue
                    }
                }
            }
            ']' => {
                if prev_char != '\\' {
                    if stack.len() > 0 {
                        let (_, closing_tag) = stack.pop().unwrap().construct();
                        output.push_str(&closing_tag);

                        continue

                    }
                } else {
                    output.pop();
                }
            }
            '~' => {
                if prev_char != '\\' {
                    output.push_str("<br />");
                    continue
                } else {
                    output.pop();
                }
            }
            '`' => {
                if prev_char != '\\' {
                    output.push_str("<div style='width:2em;display:inline-block;'></div>");
                    continue
                } else {
                    output.pop();
                }
            }
            _ => {}
        }

        if c != '\n' {
            match pen {
                Pen::Output => { output.push(c) }
                Pen::Kind => { if let Some(l) = stack.last_mut() { l.push_kind(c); } }
                Pen::Args => { if let Some(l) = stack.last_mut() { l.push_args(c); } }
            }
        }

        prev_char = c;
    }

    output
}

fn gen_page(str: String, style: &String) -> String {
    let clone = str.clone();
    let mut lines = clone.split("\n");

    let mut top = lines.next().unwrap().chars();
    let first_char = if let Some(c) = top.next() {
        c
    } else {
        ' '
    };
    let rest: String = top.collect();

    let body = lines.fold(String::new(), |acc, x| { format!("{}{}\n", acc, x) });
    
    let (title, source) = if first_char == '!' {
        (rest, body)
    } else if first_char == '\\' {
        ("".to_string(), format!("{}\n{}", rest, body))
    } else {
        ("".to_string(), str)
    };

    format!("<!DOCTYPE html><html><head><title>{}</title>\
            <link rel='stylesheet' href='{}' /></head><body>{}</body></html>",
            title.trim(), style, parse(source))
}

fn usage(prog_name: &String) {
    println!("Usage: '{}' [options...]", prog_name);
    println!(
        "\t-i, --input-dir <dir>\t\tSets folder to retrieve input from.\tDefault is '{}'.\n\
         \t-o, --output-dir <dir>\t\tSets folder to push output to.\t\tDefault is '{}'.\n\
         \t-e, --extras-dir <dir>\t\tDefines the folder extras are located.\tDefault is '{}'.\n\
         \t-s, --stylesheet <filename>\tDefines the filename of the stylesheet.\tDefault is '{}'.\n\
         \t-h, --help\t\t\tPrints this message.", INPUT_DIR, OUTPUT_DIR, EXTRAS_DIR, STYLE);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let (mut red, mut rid, mut rod, mut style) = 
        (EXTRAS_DIR.to_string(), INPUT_DIR.to_string(), OUTPUT_DIR.to_string(), STYLE.to_string());

    let mut args_iter = args.iter().enumerate();
    args_iter.next();
    if args.len() > 1 {
        for (i, a) in args_iter {
            if a == "-i" || a == "--input-dir" {
                if let Some(a) = args.get(i + 1) {
                    rid = a.clone();
                } else {
                    println!("No directory provided!\nAborting!");
                    return
                }
            } else if a == "-o" || a == "--output-dir" {
                if let Some(a) = args.get(i + 1) {
                    rod = a.clone();
                } else {
                    println!("No directory provided!\nAborting!");
                    return
                }
            } else if a == "-e" || a == "--extras-dir" {
                if let Some(a) = args.get(i + 1) {
                    red = a.clone();
                } else {
                    println!("No directory provided!\nAborting!");
                    return
                }
            } else if a == "-s" || a == "--stylesheet" {
                if let Some(a) = args.get(i + 1) {
                    style = a.clone();
                } else {
                    println!("No filename provided!\nAborting!");
                    return
                }
            } else if a == "-h" || a == "--help" {
                usage(&args[0]);
                return
            }
        }
    } else {
        println!("No arguments passed, following defaults.");
    }

    let ored = red.clone();

    red = format!("{}/{}", rod, red);
    style = format!("{}/{}", ored, style);

    let (ed, id, od) = (Path::new(&red), Path::new(&rid), Path::new(&rod));

    if !ed.exists() { fs::create_dir_all(ed).expect("Failed to create directory."); }
    if !id.exists() { fs::create_dir(id).expect("Failed to create directory."); }

    let (id_contents, od_contents): (Vec<_>, Vec<_>)
        = (fs::read_dir(id).unwrap().collect(),
           fs::read_dir(od).unwrap().collect());

    if id_contents.len() < 1 {
        println!("No pages found.\nAborting!");
        return
    } else {
        println!("Pages found.");
    }

    println!("Delete previous output? [y/N]");
    
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Failed to read input.");

    if answer.trim().to_uppercase() == "N" {
        println!("Aborting!");
        return
    }

    for f in od_contents.into_iter() {
        let file = f.unwrap();
        if file.path().is_file() {
            fs::remove_file(file.path()).expect("Failed to remove file.");
        
            println!("Deleted file: {:?}.", file.file_name());
        }
    }

    println!("Done.");

    for f in id_contents.into_iter() {
        let entry = f.unwrap();
        if entry.path().is_file() {
            let mut input = fs::File::open(entry.path()).unwrap();
            let mut contents = String::new();
            input.read_to_string(&mut contents).expect("Failed to read file.");

            let output_path = format!("./{}/{}.html", OUTPUT_DIR,
                                      entry.file_name().into_string().unwrap());

            let mut output = fs::File::create(Path::new(&output_path)).unwrap();
            output.write_all(gen_page(contents, &style)
                             .as_bytes()).expect("Failed to wrtie to file.");

            println!("Generated file: \"{}.html\".", entry.file_name().into_string().unwrap());
        }
    }

    println!("Completed!");
}
