
#![allow(warnings)]
use regex::Regex;

pub fn main() {
   
    // A string stored within its own memory space
    let hay = String::from("find");

    // A lice created easly, like a list
    let sentences = ["This is a sentence", "where you should find", "something findable", "to", "find"];
    
    // new regular expression string, and, an initialized object
    let re_pattern = r"\b(find)\b";
    let re = Regex::new(re_pattern).unwrap();

    // A vector that stores the result of a clousure applied to each element matched
    let matches: Vec<_> = sentences
        .iter()
        .filter_map(|sentence| re.find(sentence).map(|m| m.as_str()))
        .collect();
    
    println!("no. matches: {:?}", matches);
}

