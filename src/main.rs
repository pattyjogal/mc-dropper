extern crate curl;
extern crate scraper;

mod parser;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new("google.com", ".lol", ".kek");
}
