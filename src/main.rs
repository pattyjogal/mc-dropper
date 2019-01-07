//! Dropper - A Minecraft Package Manager
pub mod parser;
pub mod backend;
pub mod text_assets;

use crate::parser::PluginSearchable;
use crate::parser::PluginFetchable;
use crate::backend::PackageBackend;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new("https://dev.bukkit.org/search?search={}",
                                                ".listing",
                                                "div.results-name > a");

    match x.fetch("worldedit", "6.1.9") {
        Some(url) => println!("Install your package at: {}", url),
        None      => println!("I'm sorry! We couldn't find that version")
    };

    match PackageBackend::init() {
        Ok(_) => println!("Success! Dropper files have been all set up!"),
        Err(e) => println!("Could not complete setup: {}", e),
    };

    println!("Now that setup is done, let's validate the files:");

    match PackageBackend::validate() {
        Ok(_) => println!("All YAML looks valid to me!"),
        Err(e) => println!("Error encountered: {}", e),
    }
}
