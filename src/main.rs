//! Dropper - A Minecraft Package Manager
pub mod backend;
pub mod parser;
pub mod text_assets;

use crate::backend::PackageBackend;
use crate::parser::PluginFetchable;
use crate::parser::PluginSearchable;

fn main() {
    println!("Hello, world!");
    let x = parser::BukkitHTMLPluginParser::new(
        "https://dev.bukkit.org/search?search={}",
        ".listing",
        "div.results-name > a",
        "1.12".to_string(),
    );

    match x.enumerate_versions("worldedit") {
        Ok(Some((names, links))) => {
            for (ver, link) in names.iter().zip(links) {
                println!("{}->{}", ver, link);
            }
        }
        Ok(None) => println!("Sorry, that package was not found!"),
        Err(e) => println!("An unexpected error occured: {}", e),
    }

    match x.fetch("worldedit", "6.1.9") {
        Ok(Some(url)) => println!("Install your package at: {}", url),
        Ok(None) => println!("I'm sorry! We couldn't find that version"),
        Err(e) => println!("An unexpected error occured: {}", e),
    };

    match PackageBackend::validate() {
        Ok(_) => println!("All YAML looks valid to me!"),
        Err(e) => {
            println!("Error encountered: {}", e);
            PackageBackend::init();
        }
    }

    let pb = match PackageBackend::new(&x) {
        Ok(pb) => pb,
        Err(e) => panic!("I ran into an error: {}", e),
    };

    match pb.pkg_add("worldedit") {
        Ok(b) => match b {
            Some((name, version)) => println!("Package {} installed @ version {}!", name, version),
            None => println!("Did not install package"),
        },
        Err(e) => println!("Error while trying to add package: {}", e),
    }
}
