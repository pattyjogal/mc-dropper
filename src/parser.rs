//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit = "6.19"`.

use curl::http;
use scraper::{Html, Selector};
use std::collections::HashMap;

struct BukkitHTMLPluginParser {
    search_url: &str,
    list_selector: &str,
    item_selector: &str,
}

/// A basic plugin parser trait that is implemented with a parser, and outputs JAR file URLs.
trait PluginParser {
    /// Returns a new instance of the plugin parser
    ///
    /// # Arguments
    ///
    /// * `search_url` - A URL ending before the fragment where the parser can query for plugin search terms.
    fn new(search_url: &str) -> Self;

    /// Searches the search_url for a plugin keyword, and returns a `HashMap` of plugin names to install page URLs.
    fn search(&self, query: &str) -> HashMap<&str, &str>;

    /// Fetches a download link from a specific package name and version. Returns the package URL.
    ///
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn fetch(&self, package_name: &str, version_code: &str) -> &str;
}

/// An extenstion of the basic PluginParser that parses HTML plugin websites for their plugins.
///
/// While this parser tries not to make too many assumptions, it does assume that the package name can be deduced from the package URL in some way.
trait HTMLPluginParser: PluginParser {
    /// Returns a new instance of the HTML enabled plugin parser
    ///
    /// # Arguments
    ///
    /// * `search_url` - A URL for the search page where `{}` replaces the query position
    /// * `list_selector` - A [selector](https://www.w3schools.com/cssref/css_selectors.asp) for the search results container
    /// * `item_selector` - A selector for each item's name/link
    fn new(search_url: &str, list_selector: &str, item_selector: &str) -> Self;

    /// Takes the output of the name selector and somehow transforms it into a name that can be used to fetch the package later.
    fn transform_package_name(package_text: &str) -> &str;

    fn search(&self, query: &str) -> HashMap<&str, &str> {
        // Construct a URL that allows us to search the website
        let built_url = format!(self.search_url, query);

        // Grab the HTML text from that URL
        let response = http::handle()
            .get(built_url)
            .exec()
            .unwrap_or_else(|e| panic!("Failed to GET {} with error {}", built_url, e));
        let html = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
            panic!(
                "Could not parse response from {} with error {}",
                built_url, e
            )
        });

        // Parse the HTML text, and select the list of results from it
        let document = Html::parse_document(html);
        let results_selector = Selector::parse(self.results_selector);
        let results_container = document.select(&results_selector).next().unwrap();

        // Initialize a HashMap from package names to URLs, as well as a package link selector
        let mut pkgs_to_urls = HashMap::new();
        let link_selector = Selector::parse(self.item_selector);

        for element in results_container.select(&item_selector) {
            println!(element.value().attr("href"));
        }

        pkgs_to_urls
    }
}

/// Implement an HTMLPluginParser for Bukkit
impl HTMLPluginParser for BukkitHTMLPluginParser {
    fn new(search_url: &str, list_selector: &str, item_selector: &str) -> Self {
        BukkitHTMLPluginParser {
            search_url: search_url,
            list_selector: list_selector,
            item_selector: item_selector,
        }
    }

    fn fetch(&self, package_name: &str, version_code: &str) -> &str {
        ""
    }

    fn transform_package_name(package_text: &str) -> &str {
        ""
    }
}
