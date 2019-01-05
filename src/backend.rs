//! This module contains all the actual package management code. It supports a TOML config file,
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


/// The add function takes in a package specifier, and performs an install, as well as dumping
/// the requirement to the config file, if need be.
///
/// # Arguments
///
/// * `pkg_specifier` - A string slice that represents the package and version the user wishes
///                     to add. It should be in the package specifier format defined above.
fn pkg_add(pkg_specifier: &str) -> Result<i32, &str> {
    unimplemented!();
}
