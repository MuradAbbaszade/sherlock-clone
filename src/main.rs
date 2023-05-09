use std::env;
use std::process;
use crate::common::Url;
use reqwest::Error;
use reqwest::StatusCode;

pub mod common;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let help_message = "Usage help : cargo run 'username'";
    let args:Vec<String> = env::args().collect();
    let username = get_username(&args);
    let urls:Vec<Url> = vec![
    Url{name:String::from("GitHub"),url:String::from("https://www.github.com/")},
    Url{name:String::from("HackerRank"),url:String::from("https://hackerrank.com/")},
    Url{name:String::from("Leetcode"),url:String::from("https://leetcode.com/")}
    ];
    for url in urls {
        fetch_url(&url,&username).await?;
    }
    Ok(())
}
fn get_username(args:&Vec<String>) ->&String { 
    let help_message = "Usage help : cargo run 'username'";
    if args.len()!=2 {
        println!("{}",help_message);
        process::exit(0);
    }
    &args[1]
}
async fn fetch_url(url:&Url,username: &str) -> Result<(), Error> {
    let username_url = format!("{}{}", url.url, username);
    let response = reqwest::get(username_url).await?;
    match response.status() {
        StatusCode::OK => {
            println!("{} Found !",url.name);
        }
        StatusCode::NOT_FOUND => {
            println!("{} Not Found",url.name);
        }
        _ => {
            println!("HTTP error code: {}", response.status());
        }
    }
    Ok(())
}
