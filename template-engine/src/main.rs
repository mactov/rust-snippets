use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path: &String = &args[1];
        let mut named_args: HashMap<&String, &String> = HashMap::new();
        let mut i: usize = 2;
        while i < args.len() - 1 {
            named_args.insert(&args[i], &args[i+1]);
            i += 2;
        }
        let mut contents: String = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let re: Regex = Regex::new(r"\{\{ (.+?) \}\}").unwrap();
        let mut values_to_feed: Vec<String> = Vec::new();
        for cap in re.captures_iter(&contents) {
            if !(values_to_feed.iter().any(|n| n == &cap[1])) {
                values_to_feed.push(cap[1].to_string());
            }
        }
        for val in values_to_feed {
            let s: String = String::from("{{ ") + &val + " }}";
            contents = contents.replace(&s, named_args.get(&val).unwrap());
        }
        println!("{}", contents);
    } else {
        println!("USAGE :template-engine path_to_template_file var1 val1 var2 val2");
    }
}