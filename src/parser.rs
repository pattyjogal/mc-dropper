//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit = "6.19"`.

use curl::http;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct BukkitHTMLPluginParser {
    search_url: &'static str,
    list_selector: &'static str,
    item_selector: &'static str,
}

trait PluginSearchable {
    /// Searches the search_url for a plugin keyword, and returns a `HashMap` of plugin names to install page URLs.
    fn search(&self, query: &str) -> HashMap<&str, &str>;
}

trait PluginFetchable {
    /// Fetches a download link from a specific package name and version. Returns the package URL.
    ///
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn fetch(&self, package_name: &str, version_code: &str) -> &str;
}

trait HTMLPluginScrapable {
    /// Takes the output of the name selector and somehow transforms it into a name that can be used to fetch the package later.
    fn transform_package_name(package_text: &str) -> &str;

    /// Given a query, use the list_selector and item_selector to render a map of names to links
    fn scrape_links_from_list(
        &self,
        query: &str,
        search_url: &str,
        list_selector: &str,
        item_selector: &str,
    ) -> HashMap<&str, &str> {
        // Construct a URL that allows us to search the website
        let built_url = str::replace(search_url, "{}", query);

        // Grab the HTML text from that URL
        let response = http::handle()
            .get(built_url)
            .exec()
            .unwrap_or_else(|e| panic!("Failed to GET {} with error {}", built_url, e));
        let html = std::str::from_utf8(response.get_body()).unwrap_or_else(|e| {
            panic!(
                "Could not parse response from {} with error {}",
                built_url, e
            )
        });

        // Parse the HTML text, and select the list of results from it
        let document = Html::parse_document(html);
        let results_selector = match Selector::parse(list_selector) {
            Err(e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
            Ok(sel) => sel,
        };
        let results_container = document.select(&results_selector).next().unwrap();

        // Initialize a HashMap from package names to URLs, as well as a package link selector
        let mut pkgs_to_urls = HashMap::new();
        let link_selector = match Selector::parse(item_selector) {
            Err(e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
            Ok(sel) => sel,
        };

        for element in results_container.select(&link_selector) {
            println!(
                "{}",
                match element.value().attr("href") {
                    Some(link) => link,
                    None => "",
                }
            );
        }

        pkgs_to_urls
    }
}

impl BukkitHTMLPluginParser {
    /// Returns a new instance of the HTML enabled plugin parser
    ///
    /// # Arguments
    ///
    /// * `search_url` - A URL for the search page where `{}` replaces the query position
    /// * `list_selector` - A [selector](https://www.w3schools.com/cssref/css_selectors.asp) for the search results container
    /// * `item_selector` - A selector for each item's name/link
    pub fn new(
        search_url: &'static str,
        list_selector: &'static str,
        item_selector: &'static str,
    ) -> Self {
        BukkitHTMLPluginParser {
            search_url: search_url,
            list_selector: list_selector,
            item_selector: item_selector,
        }
    }
}

/// Add the plugin scraping capabilities
impl HTMLPluginScrapable for BukkitHTMLPluginParser {
    fn transform_package_name(package_text: &str) -> &str {
        ""
    }
}
