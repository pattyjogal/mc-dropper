extern crate reqwest;
extern crate scraper;
extern crate regex;

mod parser;

use parser::HTMLPluginScrapable;
use parser::PluginSearchable;
use parser::PluginFetchable;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new("https://dev.bukkit.org/search?search={}",
                                                ".listing",
                                                "div.results-name > a");
    let map = x.search("world");
    for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }

    match x.fetch("silkspawners", "5.0.2") {
        Some(url) => println!("Install your package at: {}", url),
        None      => println!("I'm sorry! We couldn't find that version")
    };
}
