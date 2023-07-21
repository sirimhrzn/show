use std::{
    env::{self},
    error::Error,
    fs::{self, File},
    process,
};

#[derive(Debug)]
enum GrepOptions {
    LowerCase,
    UpperCase,
}

#[derive(Debug)]
struct Grep<'a> {
    options: Vec<GrepOptions>,
    query: Vec<String>,
    args: &'a Vec<String>,
    file_path: String,
    arg_index: usize,
    flags: Vec<String>, // supported_types: i32,
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
            flags: vec!["-F".to_string(), "-U".to_string(), "-L".to_string()],
        };
        return grep;
    }
    fn file_checker(path: &String) -> Result<&'static str, &'static str> {
        let file = File::open(path);
        match file {
            Ok(e) => Ok("file path valid"),
            Err(e) => Err("Siri error bhayo"),
        }
    }
    fn search(query: &String) -> Result<(), &'static str> {
        let file = fs::read_to_string("src/sir.txt").map_err(|e| "soro")?;
        dbg!(query);
        let mut matches = Vec::new();
        for a in file.lines() {
            if a.contains("second") {
                matches.push(a);
            }
        }
        println!("{:?}", matches);
        Ok(())
    }
    fn query_parser(query: &Vec<String>) -> Result<(), &'static str> {
        let mut queries = String::new();
        for q in query {
            queries = queries + " " + &q;
        }
        let search = Self::search(&queries)?;
        // println!(" these ate {queries}");
        Ok(())
    }
    fn index_return(val: &str, ind: &'a Vec<String>) -> Result<usize, &'static str> {
        let index = ind.iter().position(|x| x == val);
        match index {
            Some(i) => Ok(i),
            None => return Err(""),
        }
    }
    fn args_parser(grep: &mut Grep) -> Result<Vec<String>, &'static str> {
        let mut query: Vec<&String> = Vec::new();
        let mut first = 0;
        if !grep.args.contains(&"-F".to_string()) {
            return Err("File path not specified");
        }
        // let que ;
        for a in grep.args {
            match a.as_str() {
                "-F" => {
                    let option_index = Grep::index_return("-F", grep.args)?;
                    if grep.args.len() - 1 == option_index {
                        return Err("No file path specified");
                    }
                    let file = grep.args[option_index + 1].to_string();
                    grep.query = grep.args[1..option_index].to_vec();
                    grep.file_path = file;
                }
                "-L" => {
                    grep.options.push(GrepOptions::LowerCase);
                }
                "-U" => {
                    grep.options.push(GrepOptions::UpperCase);
                }
                _ => {
                    let option_index = Grep::index_return("-F", grep.args)?;
                    // katy samma arguments line
                    // let mut ind = 0;
                    // if first != 0 {
                    // if ind < option_index  {
                    //       query.push(a);
                    // query.pop();
                    //     ind += 1;
                    //}
                    // }
                }
            }
            // first = 1;
        }

        let _file_path = Grep::file_checker(&grep.file_path)?;
        // query.pop();
        let _quer_p = Grep::query_parser(&grep.query);

        Ok(vec!["gee".to_string()])
    }
}
