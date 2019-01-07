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

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::{fmt, fs, io};
use std::path::Path;
use yaml_rust::YamlLoader;
use text_assets;

const CONFIG_ROOT: &'static str = "./.dropper";
const CONFIG_PATH: &'static str = "./.dropper/config.yml";
const PKG_LIST_PATH: &'static str = "./pkg.yml";

#[derive(Debug)]
pub enum ErrorKind {
    // Something when wrong while trying to parse the YAML file. Expects the filename as a param.
    YamlInvalid(String),
}

impl Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ErrorKind::YamlInvalid(s) => format!("invalid YAML syntax on file {}", s)
        })
    }
}

/// Struct to hold the configuration information for the backend
pub struct PackageBackend {
    plugin_website: String,
}

impl PackageBackend {
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
    /// # Behavior
    /// The only error this function can throw is if it detects that the config/pkg files are corrupt or
    /// malformed. The interface should handle what happens at this point (e.g. display the YML validation
    /// output, or prompt them if they wish to re-initialize)
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
    /// # Errors
    /// * [`ErrorKind::YamlInvalid`](enum.ErrorKind.html#variant.YamlInvalid) - one of the YML files is invalid
    /// * `std::io::ErrorKind::*` - an IO error occured
    pub fn validate() -> Result<(), Box<Error>> {
        PackageBackend::read_yaml_file(CONFIG_PATH)?;
        PackageBackend::read_yaml_file(PKG_LIST_PATH)?;
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
                }
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
    pub fn pkg_add(&self, pkg_specifier: &str) -> Result<bool, Box<Error>> {
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
    pub fn pkg_install(&self, pkg_specifier: &str) -> Result<bool, Box<Error>> {
        unimplemented!();
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
}
