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
}

#[derive(Debug)]
struct Grep<'a> {
    options: Vec<GrepOptions>,
    query: Vec<String>,
    args: &'a Vec<String>,
    file_path: String,
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
    fn new(options: Vec<GrepOptions>, args: &'a Vec<String>) -> Self {
        let grep = Grep {
            options,
            args,
            file_path: "".to_string(),
            query: vec!["".to_string()],
            flags: vec!["-F".to_string(), "-U".to_string(), "-L".to_string()],
        };
        return grep;
    }
    fn file_checker(&self) -> Result<(), &'static str> {
        let file = File::open(&self.file_path);
        match file {
            Ok(_) => Ok(()),
            Err(_) => Err("File not found"),
        }
    }
    fn search(&self) -> Result<(), &'static str> {
        self.file_checker()?;
        let query = &self.query_parser();
        let file = fs::read_to_string(&self.file_path);
        let mut matches = Vec::new();
        match file {
            Ok(content) => {
                for line in content.lines() {
                    if line.contains(query) {
                        matches.push(line);
                    }
                }
                if matches.is_empty() {
                    return Err("No matches found ");
                }
                println!("{:?}", matches);
                Ok(())
            }
            Err(_) => Err("Unable to read data"),
        }
    }
    fn query_parser(&self) -> String {
        self.query.join(" ")
    }
    fn index_return(&self) -> Result<usize, &'static str> {
        let index = self.args.iter().position(|x| x == &self.flags[0]);
        match index {
            Some(i) => Ok(i),
            None => return Err("Error indexing flag. No flag specified."),
        }
    }
    fn args_parser(&mut self) -> Result<Vec<String>, &'static str> {
        if !self.args.iter().any(|arg| self.flags.contains(&arg)) {
            return Err("No flags specified");
        }
        for flag in self.args {
            match flag.as_str() {
                "-F" => {
                    let option_index = self.index_return()?;
                    if self.args.len() - 1 == option_index {
                        return Err("No file path specified");
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

        let _search = self.search()?;
        Ok(vec!["gee".to_string()])
    }
}


