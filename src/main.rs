use reqwest;
use seahorse::{color};
use serde_json::{self, Value};

use std::{
    process::exit,
};

fn main(){

let mut store_word = String::new();
let source = String::from("en"); //source language
let target = String::from("km"); //target language
let words = String::from("hello"); //words for translate
let mut args: Vec<String> = Vec::new();
args.push(words);

// generate_url(args, source, target);

let url = generate_url(args, source, target);
    let v = reqwest::blocking::get(&url)
        .and_then(|resp| resp.text())
        .and_then(|body| Ok(serde_json::from_str::<Vec<Value>>(&body)))
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("Network error..."));
            exit(1);
        })
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("JSON parse error..."));
            exit(1);
        });

    match v.first() {
        Some(item) => {
            let result = item
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s[0].as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" ");
            // println!("{}", result);
            // stdout()
            //     .lock()
            //     .write_all(format!("{}\n", result).as_bytes())
            //     .unwrap();
            store_word = result;    
        }

        None => eprintln!("{}", color::red("Error...")),
    }  
    
    println!("{}", store_word);
}

fn generate_url(v: Vec<String>, source: String, target: String) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    let q = v.join(" ");
    format!(
        "{}{}{}{}{}",
        base_url,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", q).to_string()

    )
    
}