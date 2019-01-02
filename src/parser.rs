//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit = "6.19"`.

use std::collections::HashMap;

struct BukkitHTMLPluginParser {
    search_url: &str,
    list_selector: &str,
    item_selector: &str
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
trait HTMLPluginParser: PluginParser {
    /// Returns a new instance of the HTML enabled plugin parser
    ///
    /// # Arguments
    ///
    /// * `search_url` - A URL ending before the fragment where the parser can query for plugin search terms.
    /// * `list_selector` - A [selector](https://www.w3schools.com/cssref/css_selectors.asp) for the search results container
    /// * `item_selector` - A selector
    fn new(search_url: &str, list_selector: &str, item_selector: &str) -> Self;

    /// Takes the output of the name selector and somehow transforms it into a name that can be used to fetch the package later.
    fn transform_package_name(package_text: &str) -> &str;
}


/// Implement an HTMLPluginParser for Bukkit
impl HTMLPluginParser for BukkitHTMLPluginParser {

}
