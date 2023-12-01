use reqwest::Error as ReqError;
use reqwest::blocking::Client;


fn main () {
    println!("{}", get_response("https://adventofcode.com/2023/day/1/input").unwrap());
}

fn get_response(url: &str) -> Result<String, ReqError> {
    let client = Client::new();
    let response = client.get(url).send()?;
    response.text()
}