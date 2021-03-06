//! This module allows the package manager to fetch JARs from a plugin repository. The URL and package parsing behavior will be abstracted away as much as possible, since it'd be nice to have a proper API for fetching plugins, but one does not exist at the time of writing this module.
//!
//! Plugin parsers have two modi operandi: either users can search for install terms, like "World", and come back with a list of plugins to install, or they can specify a specific version, like `WorldEdit: "6.1.9"`.

use regex::Regex;
use reqwest::StatusCode;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

const BUKKIT_PKG_FORMAT_URL: &'static str =
    "https://dev.bukkit.org/projects/{}/files?filter-game-version=<>";

// A version code regular expression that allows for wildcards, and the occasional
// fourth version sub-code. (Most plugins should follow up to three, but some like WorldEdit
// don't do this for some reason)
pub const VERSION_CODE_REGEX: &'static str = r"(\d+)\.(\*|\d+)?\.?(\*|\d+)?\.?(\*|\d+)?";

#[derive(Debug)]
pub enum ErrorKind {
    // The status code was bad, and likely not by fault of user input. Website could be down,
    // or network could be configured incorrectly. Takes the status code as a param.
    RequestFailed(StatusCode),
    // The server version requested was not found for the implemented plugin website. Takes the
    // offending version code as a param.
    ServerVersionNotFound(String),
    // The version format is unknown and could not be parsed.
    BadVersioningFormat,
}

impl Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::RequestFailed(s) => format!("request failed with code {}", s),
                ErrorKind::ServerVersionNotFound(s) => {
                    format!("a plugin for server version {} not found", s)
                }
                ErrorKind::BadVersioningFormat => {
                    "plugin has a version format we cannot handle".to_string()
                }
            }
        )
    }
}

pub struct BukkitHTMLPluginParser {
    search_url: &'static str,
    list_selector: &'static str,
    item_selector: &'static str,
    minecraft_version: String,
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
    fn fetch(&self, package_name: &str, version_code: &str) -> Result<Option<String>, Box<Error>>;

    fn find_newest_version(
        &self,
        package_name: &str,
    ) -> Result<Option<(String, String)>, Box<Error>>;

    /// Provides a way to list all the versions of the package in question. Can return two Vecs
    /// of version names and links (1 : 1 in order), or if no package was found, returns `None`.
    /// *Note*: `package_name` has to be specifically formatted for the website being used. This name will be slipped into a URL to download the package in this function.
    fn enumerate_versions(
        &self,
        package_name: &str,
    ) -> Result<Option<(Vec<String>, Vec<String>)>, Box<Error>>;
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
        minecraft_version: String,
    ) -> Self {
        BukkitHTMLPluginParser {
            search_url: search_url,
            list_selector: list_selector,
            item_selector: item_selector,
            minecraft_version: minecraft_version,
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
    fn enumerate_versions(
        &self,
        package_name: &str,
    ) -> Result<Option<(Vec<String>, Vec<String>)>, Box<Error>> {
        // Construct a URL that allows us to walk the files table
        let built_url = str::replace(BUKKIT_PKG_FORMAT_URL, "{}", package_name);
        let built_url = str::replace(&built_url, "<>", &self.bukkit_mc_version_code()?);

        // Get the website content first
        let mut response = reqwest::get(&built_url)?;

        let html = match response.status() {
            // In this case, the plugin can't be found.
            StatusCode::NOT_FOUND => return Ok(None),
            status => match status.is_success() {
                true => response.text()?.to_string(),
                false => return Err(Box::new(ErrorKind::RequestFailed(status))),
            },
        };

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
                Some(link) => format!("https://dev.bukkit.org{}/download", link),
                None => "".to_string(),
            },
        );

        // Transform the list of version names to version codes
        let plugin_versions = Self::extract_version_numbers(plugin_version_names)?;

        Ok(Some((plugin_versions, plugin_version_links)))
    }

    fn find_newest_version(
        &self,
        package_name: &str,
    ) -> Result<Option<(String, String)>, Box<Error>> {
        // Get the version numbers
        let (versions, links) = match self.enumerate_versions(package_name)? {
            Some(tup) => tup,
            None => return Ok(None),
        };

        // Return a tuple of the first of each list
        Ok(Some((
            versions.first().cloned().unwrap(),
            links.first().cloned().unwrap(),
        )))
    }

    fn fetch(&self, package_name: &str, version_code: &str) -> Result<Option<String>, Box<Error>> {
        // Get the version numbers
        let (plugin_version_names, plugin_version_links) =
            match self.enumerate_versions(package_name)? {
                Some(tup) => tup,
                None => return Ok(None),
            };

        // Set up a mapping between the two above vectors
        let mut names_to_links: HashMap<String, String> = HashMap::new();
        for (name, link) in plugin_version_names.iter().zip(plugin_version_links) {
            names_to_links.insert(name.to_string(), link.to_string());
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
                    return Ok(Some(link));
                }
            }
        }

        // The version wasn't found, so we return None
        Ok(None)
    }
}

impl BukkitHTMLPluginParser {
    /// Bukkit has no defined versioning system; versions are _named_, but that doesn't
    /// help us much, since the names can include useless, inconsistent, or conflicting info.
    /// E.g. some plugins will list MC versions they are compatible with in the title, which
    /// can be figured out when we know what version number we want, but it's harder to reverse
    /// and decide what the version number actually is. So, we have to make some educated guesses
    /// using the rest of the versions to look for patterns.
    pub fn extract_version_numbers(version_list: Vec<String>) -> Result<Vec<String>, Box<Error>> {
        // One way of solving this problem is to go down the list of versions,
        // and attempt to find a strain that seems to decrement normally.
        // Admittedly, this won't fare well with version numbers that are super
        // close to the MC package versions, but that's a limitation that will
        // be considered later

        // Stores lists of version tuples that it finds
        // e.g. [(6, 1, 9, None), (1, 12, None, None)]
        let mut version_tuples: Vec<Vec<(u32, u32, Option<u32>, Option<u32>)>> = Vec::new();

        let re = Regex::new(VERSION_CODE_REGEX).unwrap();

        for version in version_list {
            // Count the matched groups (should be between 3 and 5
            // The first group is the whole match, and each subsequent is a version num
            let mut entry_versions = Vec::new();
            for groups in re.captures_iter(&version) {
                match (groups.get(1), groups.get(2)) {
                    // Push the appropriate tuple to this entry's version list
                    (Some(a), Some(b)) => entry_versions.push((
                        a.as_str().parse::<u32>()?,
                        b.as_str().parse::<u32>()?,
                        match groups.get(3) {
                            Some(s) => Some(s.as_str().parse::<u32>()?),
                            None => None,
                        },
                        match groups.get(4) {
                            Some(s) => Some(s.as_str().parse::<u32>()?),
                            None => None,
                        },
                    )),
                    // If either of the first two are null, we throw an error
                    _ => return Err(Box::new(ErrorKind::BadVersioningFormat)),
                }
            }

            version_tuples.push(entry_versions);
        }

        //for x in version_tuples.iter() {
        //  println!("{:#?}", x);
        //}

        // A quick heuristic: if each of the vectors only has len 1, then we can simply return
        // this mapping
        if version_tuples.iter().all(|x| x.len() == 1) {
            return Ok(version_tuples
                .iter()
                .map(|x| Self::stringify_version_tuple(x[0], None))
                .collect());
        }

        // Otherwise, we need to do some digging: start off by picking a position of versions.
        // We assume the plugin maker was competent enough to at least keep their versioning
        // consistent, but if not we will likely throw an error. With that in mind, if we find
        // a string of versions that decrements nicely, that's a good candidate for our versions.
        // We want to throw out strings of versions that stay the same or are inconsistent.
        let max_len = version_tuples
            .iter()
            .fold(0, |acc, x| std::cmp::max(acc, x.len()));

        let mut similar_streaks = vec![0; max_len];

        for i in 0..max_len {
            let mut version_iter = version_tuples.iter();
            let mut prev = version_iter.next().unwrap()[i];

            for tuples in version_iter {
                // It is possible that this entry does not have a tuple for this
                // column (i). If so, we increment similar_streaks either way,
                // since not being present is a good sign that it's not the version num
                if i >= tuples.len() {
                    similar_streaks[i] += 1;
                    continue;
                }

                // See if the tuple was the same as the prev tuple
                if tuples[i] == prev {
                    similar_streaks[i] += 1;
                }

                prev = tuples[i];
            }
        }

        // Get the column that had the least similarity/non-appearance
        let (col, _) = similar_streaks
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        Ok(version_tuples
            .iter()
            .map(|x| Self::stringify_version_tuple(x[col], None))
            .collect())
    }

    /// A private function to take a version tuple and stringify it. Can also take a beta version
    /// param
    fn stringify_version_tuple(
        tup: (u32, u32, Option<u32>, Option<u32>),
        beta: Option<String>,
    ) -> String {
        let mut version_code = format!("{}.{}", tup.0, tup.1);
        version_code = match tup.2 {
            Some(num) => format!("{}.{}", version_code, num),
            None => version_code,
        };

        version_code = match tup.3 {
            Some(num) => format!("{}.{}", version_code, num),
            None => version_code,
        };

        version_code = match beta {
            Some(num) => format!("{}b{}", version_code, num),
            None => version_code,
        };

        version_code
    }

    /// Bukkit has another annoyance: their filterable MC version codes are a very odd mapping.
    /// This function abstracts that away and handles it.
    fn bukkit_mc_version_code(&self) -> Result<String, ErrorKind> {
        // This will feature more versions soon
        Ok(match self.minecraft_version.as_ref() {
            "1.12" => "2020709689:6588",
            "1.11" => "2020709689:630",
            "1.10" => "2020709689:591",
            "1.9" => "2020709689:585",
            "1.8.1" => "2020709689:532",
            "1.8" => "2020709689:531",
            "CB 1.7.9-R0.2" => "2020709689:490",
            "CB 1.7.9-R0.1" => "2020709689:473",
            "CB 1.7.2-R0.3" => "2020709689:403",
            "1.7.4" => "2020709689:6391",
            "CB 1.7.2-R0.3" => "2020709689:403",
            _ => {
                return Err(ErrorKind::ServerVersionNotFound(
                    self.minecraft_version.clone(),
                ))
            }
        }
        .to_string())
    }
}
