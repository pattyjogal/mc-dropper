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

use std::path::Path;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use yaml_rust::YamlLoader;

const CONFIG_PATH: &'static str = "./.dropper/config.yml";
const YamlValidationError: Error = Error::new(ErrorKind::Other, "Invalid YAML file");

/// Some status enums to represent the outcome of each function:
///
/// * `OperationSuccess` - Everything went well and the requested action was performed
/// * `OperationNothingToDo` - No action was performed because there was nothing to do
/// * `OperationMalformed` - The package specifier was not parsable
/// * `OperationNotFound` - The requested package was not found online
enum PackageStatus {
    OperationSuccess,
    OperationNothingToDo,
    OperationMalformed,
    OperationNotFound,
}

/// Struct to hold the configuration information for the backend
struct PackageBackend {
    plugin_website: String,
}

impl PackageBackend {
    /// The initalization function for the backend. This is performed on the first run, and verified
    /// on each subsequent run.
    ///
    /// This creates a folder at the server root caled .dropper, and in it, places a default config file
    /// called `config.yml`, as well as a SQLite DB for keeping track of package installs.
    ///
    /// It also dumps a blank `pkg.yml` to the server root directory
    ///
    /// # Errors
    /// The only error this function can throw is if it detects that the config/pkg files are corrupt or
    /// malformed. The interface should handle what happens at this point (e.g. display the YML validation
    /// output, or prompt them if they wish to re-initialize)
    pub fn init() -> Result<PackageStatus, PackageStatus> {
    }

    /// Internal helper function to validate the existance of the config file
    ///
    /// # Possible Results
    /// * Ok(Some(Yaml)) - The config file exists and is returned as a Yaml
    /// * Ok(None) - The config file does not exist at all
    /// * Err(Error) - The config file exists and is invalid, or an IO error occured
    fn read_config_file() -> Result<Option<yaml_rust::Yaml>, Error> {
        let file = match File::open(CONFIG_PATH) {
            Ok(f) => f,
            Err(e) => return match e.kind() {
                // If the file couldn't be found, that's ok and we return a None
                // Otherwise, we return the other IO error that we encountered
                NotFound => Ok(None),
                _ => Err(e)
            }
        };

        let contents = String::new();
        file.read_to_string(&mut contents)?;

        // Either return the Yaml object we get (and the only first document at that),
        // or return a validation error if YamlLoader is not able to parse.
        match YamlLoader::load_from_str(&contents) {
            Ok(yaml) => Ok(Some(yaml[0])),
            Err(_e) => Err(YamlValidationError)
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
    fn pkg_add(&self, pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
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
    /// If the package specifier was invalid, or valid but not found, the Result returned will contain
    /// an error, and it will need to be handled in whatever frontend is being used.
    fn pkg_install(&self, pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
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
    fn pkg_update(&self, pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
        unimplemented!();
    }
}
