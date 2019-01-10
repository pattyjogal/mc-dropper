//! Dropper - A Minecraft Package Manager
pub mod backend;
pub mod parser;
pub mod text_assets;

use crate::backend::PackageBackend;
use crate::parser::BukkitHTMLPluginParser;
use crate::parser::PluginFetchable;
use crate::parser::PluginSearchable;

fn main() {
    let x = parser::BukkitHTMLPluginParser::new(
        "https://dev.bukkit.org/search?search={}",
        ".listing",
        "div.results-name > a",
        "1.11".to_string(),
    );

    match x.enumerate_versions("vault") {
        Ok(Some((names, links))) => {
            println!("Here is the version names to link mapping:");
            for (ver, link) in names.iter().zip(links) {
                println!("{} -> {}", ver, link);
            }

            println!("\nI found these version tags:");
            for ver in BukkitHTMLPluginParser::extract_version_numbers(names).unwrap() {
                println!("{}", ver);
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

    match pb.pkg_install("worldedit") {
        Ok(b) => match b {
            Some((name, version)) => println!("Package {} installed @ version {}!", name, version),
            None => println!("Did not install package"),
        },
        Err(e) => println!("Error while trying to add package: {}", e),
    }
}
