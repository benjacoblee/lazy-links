use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path;
use std::path::PathBuf;
use std::process;

struct Link {
    url: String,
    text: String,
}

fn is_text_file(s: &str) -> bool {
    s.ends_with(".txt") || s.ends_with(".md")
}

fn get_link_data(buf: &PathBuf) -> Vec<Link> {
    let file = File::open(buf);

    if file.is_err() {
        println!("Error: could not open file. Are you sure you provided the right path?");
        process::exit(1);
    }

    let mut reader = BufReader::new(file.unwrap());
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();
    let html = markdown::to_html(&str);
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let elements = dom.query_selector("a").unwrap();

    elements
        .into_iter()
        .map(|el| {
            let e = el.get(dom.parser());
            let link_el = e.unwrap();
            let inner_text = link_el.inner_text(dom.parser());
            let href = link_el.as_tag().unwrap().attributes().get("href");
            let url = href.unwrap().unwrap().as_utf8_str();
            Link {
                url: url.to_string(),
                text: inner_text.to_string(),
            }
        })
        .collect::<Vec<Link>>()
}

fn write_to_file(buf: &PathBuf, links: Vec<Link>) {
    let file = File::options().append(true).open(buf);

    if file.is_err() {
        println!("Error: could not open file");
        process::exit(1);
    }

    let mut file = file.unwrap();
    let write_res = write!(file, "\n**Links**\n");

    if write_res.is_err() {
        println!("Error: could not write to file");
        process::exit(1);
    }

    for link in links {
        let str = format!(
            r#"
- [{}]({})"#,
            link.text, link.url
        );
        let write_res = write!(file, "{str}");
        if write_res.is_err() {
            println!("Error: could not write to file");
            process::exit(1);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_name = args.get(1);

    if file_name.is_none() {
        println!("Error: no input provided");
        process::exit(1);
    }

    let file_name = file_name.unwrap();

    if !is_text_file(file_name) {
        println!("Error: input file must be a text file");
        process::exit(1);
    }

    let file_absolute_path = path::absolute(file_name).unwrap();
    let links = get_link_data(&file_absolute_path);

    if links.len() > 0 {
        write_to_file(&file_absolute_path, links);
    }
}
