//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit: "6.1.9"`.

use regex::Regex;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
use std::collections::HashMap;

const BUKKIT_PKG_FORMAT_URL: &'static str = "https://dev.bukkit.org/projects/{}/files";

// A version code regular expression that allows for wildcards, and the occasional
// fourth version sub-code. (Most plugins should follow up to three, but some like WorldEdit
// don't do this for some reason)
pub const VERSION_CODE_REGEX: &'static str = r"(\d+)\.?(\*|\d+)\.?(\*|\d+)\.?(\*|\d+)?";

pub struct BukkitHTMLPluginParser {
    search_url: &'static str,
    list_selector: &'static str,
    item_selector: &'static str,
}

fn extract_list_from_table(
    html: &str,
    list_selector: &str,
    item_selector: &str,
    extraction_fn: &Fn(ElementRef) -> String,
) -> Vec<String> {
    // Parse the HTML text, and select the list of results from it
    let document = Html::parse_document(&html);
    let results_selector = match Selector::parse(list_selector) {
        Err(_e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
        Ok(sel) => sel,
    };
    let results_container = document.select(&results_selector).next().unwrap();

    // Initialize a HashMap from package names to URLs, as well as a link selector
    let mut links = Vec::new();
    let link_selector = match Selector::parse(item_selector) {
        Err(_e) => panic!("Could not parse, because `{}` is an incorrectly formatted selector"),
        Ok(sel) => sel,
    };

    for element in results_container.select(&link_selector) {
        links.push(extraction_fn(element));
    }

    links
}

pub trait PluginSearchable {
    /// Searches the search_url for a plugin keyword, and returns a `HashMap` of plugin names to install page URLs.
    fn search(&self, query: &str) -> HashMap<String, String>;
}

pub trait PluginFetchable {
    /// Fetches a download link from a specific package name and version. Returns an optional package URL. If one is not found, the version lookup failed due to no version being present, or bad naming.
    ///
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn fetch(&self, package_name: &str, version_code: &str) -> Option<String>;

    fn find_newest_version(&self, package_name: &str) -> Option<(String, String)>;

    /// Provides a way to list all the versions of the package in question. Can return two Vecs
    /// of version names and links (1 : 1 in order), or if no package was found, returns `None`.
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn enumerate_versions(&self, package_name: &str) -> Option<(Vec<String>, Vec<String>)>;
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

        extract_list_from_table(
            &html,
            list_selector,
            item_selector,
            &|element: ElementRef| match element.value().attr("href") {
                Some(link) => link.to_string(),
                None => "".to_string(),
            },
        )
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
        for item in BukkitHTMLPluginParser::scrape_links_from_list(
            query,
            self.search_url,
            self.list_selector,
            self.item_selector,
        ) {
            map.insert(BukkitHTMLPluginParser::transform_package_name(&item), item);
        }

        map
    }
}

/// Add plugin fetching capabilities
impl PluginFetchable for BukkitHTMLPluginParser {
    fn enumerate_versions(&self, package_name: &str) -> Option<(Vec<String>, Vec<String>)> {
        // Construct a URL that allows us to walk the files table
        let built_url = str::replace(BUKKIT_PKG_FORMAT_URL, "{}", package_name);

        // Get the website content first
        let html = reqwest::get(&built_url)
            .unwrap_or_else(|_e| panic!("Could not GET from {}", built_url))
            .text()
            .unwrap_or_else(|_e| panic!("Could not get HTML body from {}", built_url));

        // Get a list of the names of each file link
        let plugin_version_names = extract_list_from_table(
            &html,
            ".listing",
            ".project-file-name-container > a",
            &|element: ElementRef| element.inner_html(),
        );

        // Get a parallel list of download links
        let plugin_version_links = extract_list_from_table(
            &html,
            ".listing",
            ".project-file-name-container > a",
            &|element: ElementRef| match element.value().attr("href") {
                // Need to append the download part of the link
                Some(link) => format!("{}/download", link),
                None => "".to_string(),
            },
        );

        Some((plugin_version_names, plugin_version_links))
    }


    fn find_newest_version(&self, package_name: &str) -> Option<(String, String)> {
        // Get the version numbers
        let (names, links) = self.enumerate_versions(package_name)?;

        // Return the first of each list
        Some((names.first().cloned()?, links.first().cloned()?))
    }

    fn fetch(&self, package_name: &str, version_code: &str) -> Option<String> {
        // Get the version numbers
        let (plugin_version_names, plugin_version_links) = self.enumerate_versions(package_name)?;

        // Set up a mapping between the two above vectors
        let mut names_to_links = HashMap::new();
        for (name, link) in plugin_version_names.iter().zip(plugin_version_links) {
            names_to_links.insert(name.to_string(), format!("https://dev.bukkit.org{}", link));
        }

        // Set up a regular expression that catches version numbers
        // From https://stackoverflow.com/questions/82064/a-regex-for-version-number-parsing
        let re = Regex::new(VERSION_CODE_REGEX).unwrap();

        // The outer loop goes down each version-to-link pair, and the inner loop
        // looks through all of the version numbers found in the version name to see
        // if the one we want shows up. This is somewhat flawed, since some people will
        // put MC server versions in their version names, but this solution should have the
        // highest hit rate.
        for (name, link) in names_to_links {
            for groups in re.captures_iter(&name) {
                if &groups[0] == version_code {
                    return Some(link);
                }
            }
        }

        // The version wasn't found, so we return None
        None
    }
}
