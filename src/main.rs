extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>
}

fn deserialize <'a, T> (serialized_data: &'a str) -> std::option::Option<T> where T: Deserialize <'a> {
    match serde_json::from_str::<T>(serialized_data) {
        Ok(data) => Some(data),
        Err(_) => None
    }
}

fn main() {
    let json = r#"
    {
        "title": "JSON API paints my bikeshed!",
        "author": "Jordan Harband",
        "paragraphs": [
            {
                "content": "This is where my bike lived."
            },
            {
                "content": "And this is where my bike bought something."
            },
            {
                "content": "And this is where my bike bought something too."
            },
            {
                "content": "And this is where my bike lived."
            }
        ]
    }"#;
    let json = json.trim();
    match deserialize::<Article>(json) {
        Some(data) => println!("Here is the serialized data:\n{}\nAnd here is the deserialized data:\n{:#?}", json, data),
        None => eprintln!("There was a problem reading the data!")
    }
}
