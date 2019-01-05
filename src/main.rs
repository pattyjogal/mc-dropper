extern crate reqwest;
extern crate scraper;
extern crate regex;
extern crate argparse;

mod parser;
mod backend;

use parser::PluginSearchable;
use parser::PluginFetchable;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new("https://dev.bukkit.org/search?search={}",
                                                ".listing",
                                                "div.results-name > a");

    match x.fetch("worldedit", "6.1.9") {
        Some(url) => println!("Install your package at: {}", url),
        None      => println!("I'm sorry! We couldn't find that version")
    };
}
