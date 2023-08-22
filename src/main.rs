
mod args;

use args::CommandType;
use args::ConfigArgs;
use clap::{Parser};
use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn combine(subdomains: Vec<String>, keywords: Vec<String>, separators: Vec<String>) -> Vec<String> {
    /* Combines lists of subdomains, keywords, and separators.
     */
    for subdomain in subdomains.iter() {
        println!("{}", subdomain);
    }
    let it = subdomains.iter().cartesian_product(keywords);
    let mut result: Vec<String> = Vec::new();

    for (subdomain, keyword) in it {
        for separator in separators.iter() {
            result.push(format!("{}{}{}", keyword, separator, subdomain));
            result.push(format!("{}{}{}", subdomain, separator, keyword));
        }
    }

    result
}


fn print_vec(vector: Vec<String>) {
    for item in &vector {
        println!("{}", item);
    }
}

fn main() {
    let args: ConfigArgs = ConfigArgs::parse();

    match args.command {
        CommandType::Combine(cmd_args) => {
            let subdomains = read_file(cmd_args.subdomains)
                .expect("Error opening file: Subdomains.");
            
    
            let keywords = read_file(cmd_args.keywords)
                .expect("Error opening file: Keywords.");

            let separators =
                read_file(cmd_args.separators)
                .expect("Error opening file: Separators.");

            print_vec(combine(subdomains, keywords, separators));
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_file() {
        let subdomains_file = "testing/subdomains.txt".to_string();
        read_file(subdomains_file).expect("Read test failed.");
    }

    #[test]
    fn test_combine() {
        let subdomains_file = "testing/subdomains.txt".to_string();
        let subdomains = read_file(subdomains_file)
            .expect("Error opening file: Subdomains.");

        let keywords_file = "testing/keywords.txt".to_string();
        let keywords = read_file(keywords_file)
            .expect("Error opening file: Keywords.");

        let separators_file = "testing/separators.txt".to_string();
        let separators =  read_file(separators_file)
            .expect("Error opening file: Separators.");

        combine(subdomains, keywords, separators);
    }
}
