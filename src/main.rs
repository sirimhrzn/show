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
                println!("No flags specified. Please run --help for info \nThis version only supports \n \n show [QUERY] -f [FILE_PATH] \n \n \n   \"-f\": Search query in the file ")
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
            flags: vec!["-f".to_string(), "-U".to_string(), "-L".to_string()],
        }
    }
    fn file_checker(&self) -> bool {
        let file = File::open(&self.file_path);
        match file {
            Ok(_) => true,
            Err(_) => false,
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
        if !self.file_checker() {
            return Err(GrepError::FileNotFound);
        }
        let query = self.query_parser();
        let file = fs::read_to_string(&self.file_path);
        let mut match_count: usize = 0;
        match file {
            Ok(content) => {
                for (i, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        match_count += 1;
                        if i < 10 {
                            println!("{}:  {}", &i, &line);
                        } else {
                            println!("{}: {}", &i, &line);
                        }
                    }
                }
                if match_count == 0 {
                    return Err(GrepError::QueryNotFound);
                }
                println!("{} matches found", &match_count);
                Ok(())
            }
            Err(_) => Err(GrepError::ReadFailed),
        }
    }
    fn query_parser(&self) -> String {
        self.query.join(" ")
    }
    fn index_return(&self, flag: usize) -> Result<usize, GrepError> {
        let index = self.args.iter().position(|x| x == &self.args[flag]);
        match index {
            Some(i) => Ok(i),
            None => return Err(GrepError::CustomError("Invalid index")),
        }
    }
    fn cat(&self) -> Result<String, GrepError> {
        match fs::read_to_string(&self.file_path) {
            Ok(data) => Ok(data),
            Err(_) => Err(GrepError::ReadFailed),
        }
    }
    fn args_parser(&mut self) -> Result<Vec<String>, GrepError> {
        if self.args.len() == 1 {
            return Err(GrepError::NoFlagSpecified);
        }
        self.file_path = self.args[1].clone();
        if !self.args.iter().any(|arg| self.flags.contains(&arg)) && self.file_checker() {
            if self.args.len() > 2 {
                return Err(GrepError::NoFlagSpecified);
            }
            let cat = self.cat()?;
            println!("{cat}");
            return Ok(vec!["".to_string()]);
        }
        for (i, flag) in self.args.iter().enumerate() {
            match flag.as_str() {
                "--help" => {
                    println!("This version only supports \n \n \n show [QUERY] -f [FILE_PATH] \n \"-f\": Search query in the file ");
                }
                "-f" => {
                    let option_index = self.index_return(i)?;
                    if self.args.len() - 1 == option_index {
                        return Err(GrepError::NoFilePathSpecified);
                    }
                    self.file_path = self.args[option_index + 1].to_string();
                    self.query = self.args[1..option_index].to_vec();
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
