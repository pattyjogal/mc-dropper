//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit = "6.19"`.

use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashMap;

pub struct BukkitHTMLPluginParser {
    search_url: &'static str,
    list_selector: &'static str,
    item_selector: &'static str,
}

pub trait PluginSearchable {
    /// Searches the search_url for a plugin keyword, and returns a `HashMap` of plugin names to install page URLs.
    fn search(&self, query: &str) -> HashMap<String, String>;
}

pub trait PluginFetchable {
    /// Fetches a download link from a specific package name and version. Returns the package URL.
    ///
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn fetch(&self, package_name: &str, version_code: &str) -> &str;
}

pub trait HTMLPluginScrapable {
    /// Takes the output of the name selector and somehow transforms it into a name that can be used to fetch the package later.
    /// By default, this just returns the package text
    fn transform_package_name(package_text: &str) -> String {
        package_text.to_string()
    }

    /// Given a query, use the list_selector and item_selector to render a map of names to links
    fn scrape_links_from_list(
        query: &str,
        search_url: &str,
        list_selector: &str,
        item_selector: &str,
    ) -> Vec<String> {
        // Construct a URL that allows us to search the website
        let built_url = str::replace(search_url, "{}", query);

        // Grab the HTML text from that URL
        let html = reqwest::get(&built_url)
            .unwrap_or_else(|e| panic!("Could not GET from {}", built_url))
            .text()
            .unwrap_or_else(|e| panic!("Could not get HTML body from {}", built_url));

        // Parse the HTML text, and select the list of results from it
        let document = Html::parse_document(&html);
        let results_selector = match Selector::parse(list_selector) {
            Err(e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
            Ok(sel) => sel,
        };
        let results_container = document.select(&results_selector).next().unwrap();

        // Initialize a HashMap from package names to URLs, as well as a package link selector
        let mut pkgs_to_urls = Vec::new();
        let link_selector = match Selector::parse(item_selector) {
            Err(e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
            Ok(sel) => sel,
        };

        for element in results_container.select(&link_selector) {
            let link = match element.value().attr("href") {
                Some(link) => link,
                None => "",
            };
            pkgs_to_urls.push(link.to_string());
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
    fn transform_package_name(package_text: &str) -> String {
        let re = Regex::new(r"^/projects/(\w.*)\?").unwrap();
        // Return the captured project name
        re.captures_iter(package_text).next().unwrap()[1].to_string()
    }
}

/// Add plugin searching capabilities
impl PluginSearchable for BukkitHTMLPluginParser {
    fn search(&self, query: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for item in BukkitHTMLPluginParser::scrape_links_from_list(query,
                                           self.search_url,
                                           self.list_selector,
                                           self.item_selector) {
            map.insert(BukkitHTMLPluginParser::transform_package_name(&item), item);
        }

        map
    }
}
