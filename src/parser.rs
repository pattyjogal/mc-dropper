//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit = "6.19"`.

use std::collections::HashMap;

/// A basic plugin parser trait that is implemented with a parser, and outputs JAR file URLs.
trait PluginParser {
    /// Returns a new instance of the plugin parser
    ///
    /// # Arguments
    ///
    /// * `search_url` - A URL ending before the fragment where the parser can query for plugin search terms.
    fn new(search_url: &str) -> Self;

    /// Searches the search_url for a plugin keyword, and returns a `HashMap` of plugin names to install page URLs.
    fn search(query: &str) -> HashMap<&str, &str>;

    /// Fetches a download link from a specific package name and version. Returns the package URL
    ///
    /// This should internally use `search` to get the correct URL for the package.
    fn fetch(package_name: &str, version_code: &str) -> &str;

}
