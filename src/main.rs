extern crate reqwest;
extern crate scraper;
extern crate regex;

mod parser;

use parser::HTMLPluginScrapable;
use parser::PluginSearchable;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new("https://dev.bukkit.org/search?search={}",
                                                ".listing",
                                                "div.results-name > a");
    let map = x.search("world");
    for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }
}
