//! This module contains all the actual package management code. It supports a YAML config file,
//! as well as simple package manager operations like add, update, remove, clean/purge, install, etc.
//! This will use a little SQLite interface in order to keep track of what's been installed such
//! that package updating and removal will be easy.

//! # Package Specification
//! One of the key points of this package manager is that users can specifiy an exact version,
//! or if they want the newest major/minor/patch release.
//!
//! ## Examples
//! Note: the left version is what would be in the pkg.yml; the right version is what to type in the CLI
//! * Exact Version: `WorldEdit: 6.1.9` / `WorldEdit@6.1.9`
//! * Newest Patch: `WorldEdit: 6.1.*` / `WorldEdit@6.1.*`
//! * Newest Minor: `WorldEdit: 6.*` / `WorldEdit@6.*`
//! * Newest Major (Newest release): `WorldEdit: *` / `WorldEdit`

use crate::parser::VERSION_CODE_REGEX;
use crate::parser::{PluginFetchable, PluginSearchable};
use crate::text_assets;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{copy, Read, Write};
use std::path::Path;
use std::{fmt, fs, io};
use yaml_rust::YamlLoader;

const CONFIG_ROOT: &'static str = "./.dropper";
const CONFIG_PATH: &'static str = "./.dropper/config.yml";
const PKG_LIST_PATH: &'static str = "./pkg.yml";
const DOWNLOAD_DIR: &'static str = "./plugins";

const VERSION_SPLIT_CHAR: char = '@';

#[derive(Debug)]
pub enum ErrorKind {
    // Something when wrong while trying to parse the YAML file. Expects the filename as a param.
    YamlInvalid(String),
    // The supplied package specifier doesn't match any of the possible formats. Expects the bad
    // package specifier as a param.
    PkgSpecInvalid(String),
    // The config file was missing key(s) that we require. Takes the missing key as an arg
    ConfigInvalid(String),
    // The config file itself is missing!
    ConfigMissing,
}

impl Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::YamlInvalid(s) => format!("invalid YAML syntax on file {}", s),
                ErrorKind::PkgSpecInvalid(s) => format!("'{}' is not a valid package specifier", s),
                ErrorKind::ConfigInvalid(s) => format!("param '{}' missing from config", s),
                ErrorKind::ConfigMissing => "the config file was not found!".to_string(),
            }
        )
    }
}

/// Struct to hold the configuration information for the backend
pub struct PackageBackend<'a> {
    pub plugin_website: String,
    pub package_parser: &'a PluginFetchable,
    pub server_version: String,
}

impl<'a> PackageBackend<'a> {
    /// Creates a new backend instance. This is performed on each command, or on each app startup.
    /// Takes in a package parser to use for feeding the backend information.
    ///
    /// # Errors
    /// Any of these errors means that some part of the configuration is missing, and as such,
    /// we cannot reliable construct a backend. The frontend should handle the user's next
    /// action, as `PackageBackend::init` is destructive, and needs confirmation.
    /// * [`ErrorKind::YamlInvalid`](enum.ErrorKind.html#variant.YamlInvalid) - one of the YML files is invalid
    /// * `std::io::ErrorKind::*` - an IO error occured
    pub fn new(package_parser: &'a PluginFetchable) -> Result<PackageBackend<'a>, Box<Error>> {
        // Check if the config environment is valid
        PackageBackend::validate()?;

        // Read the config - we can expect it to exist because validate checks for
        // its existance.
        let config_yml = PackageBackend::read_yaml_file(CONFIG_PATH)?.unwrap();

        Ok(PackageBackend {
            plugin_website: config_yml[0]["plugin_website"]
                .clone()
                .into_string()
                .unwrap(),
            package_parser: package_parser,
            server_version: config_yml[0]["server_version"]
                .clone()
                .into_string()
                .unwrap(),
        })
    }

    /// The initalization function for the backend. This is performed only on the first run, or if the .dropper folder is ever deleted
    ///
    /// This creates a folder at the server root caled .dropper, and in it, places a default config file
    /// called `config.yml`, as well as a SQLite DB for keeping track of package installs.
    ///
    /// It also dumps a blank `pkg.yml` to the server root directory if it does not exist yet.
    ///
    /// # Warning
    /// This command is by design destructive! It will kill the config folder, along with its files,
    /// so it is advised to prompt the user before running this! The interface should check to see if
    /// a non-empty `.dropper` exists before running this, prompting the user if so.
    ///
    /// # Errors
    /// * `std::io::ErrorKind::*` - an IO error occured
    pub fn init() -> Result<(), Box<Error>> {
        // Create the directory for the config files
        if Path::new(CONFIG_ROOT).exists() {
            fs::remove_dir_all(CONFIG_ROOT)?;
        }
        fs::create_dir(CONFIG_ROOT)?;

        // Dump a default config file in there
        let mut config = File::create(CONFIG_PATH)?;
        config.write_all(text_assets::CONFIG_YAML_DEFAULT);

        // Create a pkg.yml if one does not exist yet
        let pkg_list = OpenOptions::new()
            .write(true)
            .create(true)
            .open(PKG_LIST_PATH)?;

        Ok(())
    }

    /// Ensures that the config files both exist and can be read
    ///
    /// # Behavior
    /// The only error this function can throw is if it detects that the config/pkg files are corrupt or
    /// malformed. The interface should handle what happens at this point (e.g. display the YML validation
    /// output, or prompt them if they wish to re-initialize)
    ///
    /// # Errors
    /// * [`ErrorKind::YamlInvalid`](enum.ErrorKind.html#variant.YamlInvalid) - one of the YML files is invalid
    /// * `std::io::ErrorKind::*` - an IO error occured
    pub fn validate() -> Result<(), Box<Error>> {
        let config = match PackageBackend::read_yaml_file(CONFIG_PATH)? {
            Some(c) => c,
            None => return Err(Box::new(ErrorKind::ConfigMissing)),
        };
        // Read all of the fields we need, and ensure they can be parsed into the
        // right type.
        let config_doc = &config[0];

        match config_doc["server_version"].clone().into_string() {
            Some(_) => {}
            None => {
                return Err(Box::new(ErrorKind::ConfigInvalid(
                    "server_version".to_string(),
                )))
            }
        }

        match config_doc["plugin_website"].clone().into_string() {
            Some(_) => {}
            None => {
                return Err(Box::new(ErrorKind::ConfigInvalid(
                    "plugin_website".to_string(),
                )))
            }
        }

        // No need to valdate Some/None for pkg: it doesn't _need_ to exist for all
        // operations (like install), and it will be created for other ops (like add)
        let pkg = PackageBackend::read_yaml_file(PKG_LIST_PATH)?;
        Ok(())
    }

    /// Internal helper function to validate the existance of a YAML file
    ///
    /// # Possible Results
    /// * Ok(Some(Vec<Yaml>)) - The config file exists and is returned as a YAML doc list
    /// * Ok(None) - The config file does not exist at all
    /// * Err(Error) - The config file exists and is invalid, or an IO error occured
    ///
    /// # Errors
    /// * [`ErrorKind::YamlInvalid`](enum.ErrorKind.html#variant.YamlInvalid) - one of the YML files is invalid
    /// * `std::io::ErrorKind::*` - an IO error occured
    fn read_yaml_file(path: &'static str) -> Result<Option<Vec<yaml_rust::Yaml>>, Box<Error>> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return match e.kind() {
                    // If the file couldn't be found, that's ok and we return a None
                    // Otherwise, we return the other IO error that we encountered
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(Box::new(e)),
                };
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Either return the Yaml object we get (and the only first document at that),
        // or return a validation error if YamlLoader is not able to parse.
        match YamlLoader::load_from_str(&contents) {
            Ok(yaml) => Ok(Some(yaml)),
            Err(_e) => Err(Box::new(ErrorKind::YamlInvalid(path.to_string()))),
        }
    }

    /// The add function takes in a package specifier, and performs an install, as well as dumping
    /// the requirement to the config file, if need be.
    ///
    /// # Arguments
    ///
    /// * `pkg_specifier` - A string slice that represents the package and version the user wishes
    ///                     to add. It should be in the package specifier format defined above.
    ///
    /// # Errors
    /// If the package specifier was invalid, or valid but not found, the Result returned will contain
    /// an error, and it will need to be handled in whatever frontend is being used.
    pub fn pkg_add(&self, pkg_specifier: &str) -> Result<Option<(String, String)>, Box<Error>> {
        unimplemented!();
    }

    /// The installer function which takes in a package specifier and installs that package to the user's
    /// plugin directory.
    ///
    /// # Arguments
    ///
    /// * `pkg_specifier` - A string slice that represents the package and version the user wishes
    ///                     to add. It should be in the package specifier format defined above.
    ///
    /// # Errors
    /// *
    pub fn pkg_install(&self, pkg_specifier: &str) -> Result<Option<(String, String)>, Box<Error>> {
        // Parse the package specifier
        let (pkg_url, name, version) =
            match Self::parse_package_specifier(pkg_specifier.to_string())? {
                // A version was specified: fetch that specific version
                (name, Some(version)) => match self.package_parser.fetch(&name, &version)? {
                    Some(link) => (link, name, version),
                    None => return Ok(None),
                },
                // No version was specified: get the newest version
                (name, None) => match self.package_parser.find_newest_version(&name)? {
                    Some((version, link)) => (link, name, version),
                    None => return Ok(None),
                },
            };

        let mut response = reqwest::get(&pkg_url)?;

        let mut plugin_file = {
            let filename = format!("{}/{}@{}.jar", DOWNLOAD_DIR, name, version);
            File::create(filename)?
        };
        copy(&mut response, &mut plugin_file);
        Ok(Some((name, version)))
    }

    /// The update function which takes in a package name, checks to see if it's been installed, and
    /// by default installs the newest version according to the user's pkg.yml.
    ///
    /// # Arguments
    ///
    /// * `pkg_name` - A string slice that represents the package name that the user wishes to update.
    ///
    /// # Errors
    /// If the package name specified is not installed, then the Result
    /// will contain an appropriate error, and will need to be handled with whatever frontend is being
    /// used.
    ///
    /// Additionally, this function can return a `OperationNothingToDo` if the package is already  up to date.
    pub fn pkg_update(&self, pkg_specifier: &str) -> Result<bool, Box<Error>> {
        unimplemented!();
    }

    /// An internal function to parse out the package name and version from a package specifier
    ///
    /// # Arguments
    /// * `pkg_specifier` - A string slice that represents the package and version the user wishes
    ///                     to add. It should be in the package specifier format defined above.
    ///
    /// # Errors
    /// * [`ErrorKind::PkgSpecInvalid`](enum.ErrorKind.html#variant.PkgSpecInvalid) - the package specifier was invalid
    ///
    /// # Non Error Return Value
    /// A tuple containing the package name and an option of version code. If none, assume the newest
    /// package is acceptable.
    fn parse_package_specifier(
        pkg_specifier: String,
    ) -> Result<(String, Option<String>), ErrorKind> {
        let name_re = Regex::new(r"^\w+$").unwrap();
        if pkg_specifier.contains(VERSION_SPLIT_CHAR) {
            // A version was specified along with the package
            let components = pkg_specifier
                .split(VERSION_SPLIT_CHAR)
                .collect::<Vec<&str>>();
            // Anything more than two components means that one too many separators appeared
            match components.len() {
                2 => {
                    let version_re = Regex::new(VERSION_CODE_REGEX).unwrap();

                    if !name_re.is_match(&components[0]) {
                        return Err(ErrorKind::PkgSpecInvalid(pkg_specifier));
                    }

                    if !version_re.is_match(&components[1]) {
                        return Err(ErrorKind::PkgSpecInvalid(pkg_specifier));
                    }

                    // At this point, the components are valid and can be passed out
                    Ok((components[0].to_string(), Some(components[1].to_string())))
                }
                // More than two components were found
                _ => Err(ErrorKind::PkgSpecInvalid(pkg_specifier)),
            }
        } else {
            // No version was specified along with the package
            // Ensure that the package name is just one valid word
            match name_re.is_match(&pkg_specifier) {
                true => Ok((pkg_specifier, None)),
                false => Err(ErrorKind::PkgSpecInvalid(pkg_specifier)),
            }
        }
    }
}
