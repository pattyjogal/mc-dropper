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
fn init() -> Result<PackageStatus, PackageStatus> {
    unimplemented!();
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
fn pkg_add(pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
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
fn pkg_install(pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
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
fn pkg_update(pkg_specifier: &str) -> Result<PackageStatus, PackageStatus> {
    unimplemented!();
}
