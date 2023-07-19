use std::{
    env::{self},
    error::Error,
    process,
};

#[derive(Debug)]
enum GrepOptions {
    LowerCase,
    UpperCase,
    Default,
}

#[derive(Debug)]
struct Grep<'a> {
    options: Vec<GrepOptions>,
    query: Vec<String>,
    args: &'a Vec<String>,
    file_path: String,
    arg_index: usize,
    // supported_types: i32,
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let values: Vec<String> = args.collect();
    let mut grepper = Grep::new(vec![], &values);

    if let Err(e) = Grep::args_parser(&mut grepper) {
        println!("{e}");
        process::exit(1);
    }
    Ok(())
}

impl<'a> Grep<'a> {
    fn new(options: Vec<GrepOptions>, args: &'a Vec<String>) -> Grep {
        let grep = Grep {
            options,
            args,
            file_path: "".to_string(),
            query: vec!["".to_string()],
            arg_index: 0,
        };
        return grep;
    }
    fn query_parser(query: Vec<&String>) {
        let mut queries = String::new();
        for q in query {
            queries = queries + " " + &q;
        }
        println!("{queries}");
    }
    fn index_return(val: &str, ind: &'a Vec<String>) -> Result<usize, &'static str> {
        let index = ind.iter().position(|x| x == val);
        match index {
            Some(i) => Ok(i),
            None => return Err("No file path specified after F"),
        }
    }
    fn args_parser(grep: &mut Grep) -> Result<Vec<String>, &'static str> {
        let mut query: Vec<&String> = Vec::new();

        for a in grep.args {
            match a.as_str() {
                "-F" => {
                    let option_index = Grep::index_return("-F", grep.args)?;
                    if grep.args.len() < (option_index + 1) {
                        grep.file_path = grep.args[option_index + 1].to_string();
                    }
                }
                "-L" => {
                    // let option_index = Grep::index_return("-L", grep.args.clone())?;
                    grep.options.push(GrepOptions::LowerCase);
                }
                "-U" => {
                    grep.options.push(GrepOptions::UpperCase);
                }
                _ => {
                    query.push(a);
                }
            }
        }
        let quer_p = Grep::query_parser(query);
        println!("{:?}", grep.options);
        Ok(vec!["gee".to_string()])
    }
}
