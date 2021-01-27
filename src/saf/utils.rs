use std::path;

use super::constants::{INDEX_EXTENSION, POSITION_EXTENSION, VALUE_EXTENSION};

/// Returns the stem of a SAF file path.
///
/// # Examples
///
/// ```
/// use angsd_io::saf::utils::stem;
///
/// let path = "path/to/population.saf.gz";
///
/// assert_eq!(stem(path), Some("population".into()));
/// ```
pub fn stem<P>(path: P) -> Option<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    Some(prefix(path)?.file_stem()?.into())
}

/// Returns the prefix of a SAF file path.
///
/// # Examples
///
/// ```
/// use angsd_io::saf::utils::prefix;
///
/// let path = "path/to/population.saf.gz";
///
/// assert_eq!(prefix(path), Some("path/to/population".into()));
/// ```
pub fn prefix<P>(path: P) -> Option<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    let string = path
        .as_ref()
        .to_str()
        .expect("cannot convert path to string")
        .to_string();

    let extensions = vec![INDEX_EXTENSION, VALUE_EXTENSION, POSITION_EXTENSION];

    extensions
        .iter()
        .find(|x| string.ends_with(*x))
        .map(|x| string.trim_end_matches(*x).trim_end_matches(".").into())
}
