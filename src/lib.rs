mod file_criteria;
mod file_utils;

use file_criteria::FindCriteria;

#[derive(Debug)]
pub enum FindError {
    InvalidCriteria(String),
    NotFound(String),
    NotADirectory(String),
}
impl std::fmt::Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindError::InvalidCriteria(e) => write!(f, "Invalid criteria: {}", e),
            FindError::NotFound(p) => write!(f, "Path not found: {}", p),
            FindError::NotADirectory(p) => write!(f, "Path is not a directory: {}", p),
        }
    }
}
impl std::error::Error for FindError {}

pub fn find(path: &str, criteria: &FindCriteria) -> Result<Vec<String>, FindError> {
    file_criteria::validate(criteria).map_err(|err| err)?;

    let research_path = std::path::Path::new(path);

    if !research_path.exists() {
        return Err(FindError::NotFound(path.to_string()));
    }

    if !research_path.is_dir() {
        return Err(FindError::NotADirectory(path.to_string()));
    }

    let mut found_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(research_path) {
        for entry_result in entries {
            if let Ok(entry) = entry_result {
                let file_path = entry.path();
                if file_path.is_dir() && criteria.recursive {
                    if let Some(subdirectory_path) = file_path.to_str() {
                        if let Ok(subdirectory_files) = find(subdirectory_path, criteria) {
                            found_files.extend(subdirectory_files);
                        }
                    }
                } else if file_path.is_file() && file_utils::accept(&entry, criteria) {
                    if let Some(file_str) = file_path.to_str() {
                        found_files.push(file_str.to_string());
                    }
                }
            }
        }
    }

    Ok(found_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn none_existing_path() {
        let criteria = FindCriteria::new();
        let result = find("tests_files/empty2", &criteria);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Path not found: tests_files/empty2"
        );
    }

    #[test]
    fn path_is_a_file() {
        let criteria = FindCriteria::new();
        let result = find("tests_files/file", &criteria);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Path is not a directory: tests_files/file"
        );
    }

    #[test]
    fn validate_file_size_min_greater_than_max() {
        let criteria = FindCriteria::new()
            .file_size_minimum(200)
            .file_size_maximum(100);
        let result = file_criteria::validate(&criteria);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid criteria: Minimum file size cannot be greater than maximum file size"
        );
    }

    #[test]
    fn all_files() {
        let criteria = FindCriteria::new();
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }

    #[test]
    fn not_recursive_files() {
        let criteria = FindCriteria::new().recursive(false);
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn search_name() {
        let criteria = FindCriteria::new().file_name(OsString::from("f"));
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_png() {
        let criteria = FindCriteria::new().file_extension(OsString::from("png"));
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_size_min() {
        let criteria = FindCriteria::new().file_size_minimum(89534);
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_size_max() {
        let criteria = FindCriteria::new().file_size_maximum(89534);
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn search_multiple_criteria() {
        let criteria = FindCriteria::new()
            .file_name(OsString::from("image"))
            .file_extension(OsString::from("png"))
            .file_size_minimum(89)
            .file_size_maximum(100000);
        let result = find("tests_files", &criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
