use std::{
    env,
    error::Error,
    fs::{self, File},
    process,
};
#[derive(Debug)]
enum GrepOptions {
    LowerCase,
    UpperCase,
    Default,
}

#[derive(Debug)]
enum GrepError {
    FileNotFound,
    QueryNotFound,
    ReadFailed,
    NoQuerySpecified,
    NoFilePathSpecified,
    NoFlagSpecified,
    CustomError(&'static str),
}

#[derive(Debug)]
struct Grep<'a> {
    options: Vec<GrepOptions>,
    query: Vec<String>,
    args: &'a Vec<String>,
    file_path: String,
    flags: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let values: Vec<String> = args.collect();
    let mut grepper = Grep::new(vec![GrepOptions::Default], &values);

    if let Err(e) = Grep::args_parser(&mut grepper) {
        match e {
            GrepError::FileNotFound => println!("File not found"),
            GrepError::QueryNotFound => println!("No match found for the given query"),
            GrepError::ReadFailed => println!("Failed reading document.Invalid file type"),
            GrepError::NoQuerySpecified => println!("No query given. Please run --help for info"),
            GrepError::NoFilePathSpecified => println!("No file path given "),
            GrepError::NoFlagSpecified => {
                println!("No flags specified. Please run --help for info")
            }
            GrepError::CustomError(error) => println!("{error}"),
        }
        process::exit(1);
    }
    Ok(())
}

impl<'a> Grep<'a> {
    fn new(options: Vec<GrepOptions>, args: &'a Vec<String>) -> Self {
        Grep {
            options,
            args,
            file_path: "".to_string(),
            query: vec!["".to_string()],
            flags: vec!["-F".to_string(), "-U".to_string(), "-L".to_string()],
        }
    }
    fn file_checker(&self) -> Result<(), GrepError> {
        let file = File::open(&self.file_path);
        match file {
            Ok(_) => Ok(()),
            Err(_) => Err(GrepError::FileNotFound),
        }
    }
    fn search(&mut self) -> Result<(), GrepError> {
        if self.query.is_empty()
            && self
                .options
                .iter()
                .any(|a| matches!(a, GrepOptions::Default))
        {
            return Err(GrepError::NoQuerySpecified);
        }
        self.file_checker()?;
        let query = self.query_parser();
        let file = fs::read_to_string(&self.file_path);
        let mut matches = Vec::new();
        match file {
            Ok(content) => {
                for line in content.lines() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        matches.push(line);
                    }
                }
                if matches.is_empty() {
                    return Err(GrepError::QueryNotFound);
                }
                println!(" {} matches found", matches.len());
                println!("{:?}", matches);
                Ok(())
            }
            Err(_) => Err(GrepError::ReadFailed),
        }
    }
    fn query_parser(&self) -> String {
        self.query.join(" ")
    }
    fn index_return(&self) -> Result<usize, GrepError> {
        let index = self.args.iter().position(|x| x == &self.flags[0]);
        match index {
            Some(i) => Ok(i),
            None => return Err(GrepError::CustomError("Invalid index")),
        }
    }
    fn args_parser(&mut self) -> Result<Vec<String>, GrepError> {
        if !self.args.iter().any(|arg| self.flags.contains(&arg)) {
            return Err(GrepError::NoFlagSpecified);
        }
        for flag in self.args {
            match flag.as_str() {
                "-F" => {
                    let option_index = self.index_return()?;
                    if self.args.len() - 1 == option_index {
                        return Err(GrepError::NoFilePathSpecified);
                    }
                    let file = self.args[option_index + 1].to_string();
                    self.query = self.args[1..option_index].to_vec();
                    self.file_path = file;
                }
                "-L" => {
                    self.options.push(GrepOptions::LowerCase);
                }
                "-U" => {
                    self.options.push(GrepOptions::UpperCase);
                }
                _ => {}
            }
        }

        self.search()?;
        Ok(vec!["gee".to_string()])
    }
}
