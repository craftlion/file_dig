mod file_criteria;
mod file_utils;

use file_criteria::FindCriteria;

pub fn find(path: &str, criteria: &FindCriteria) -> Result<Vec<String>, std::io::Error> {
    file_criteria::validate(criteria)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    let research_path = std::path::Path::new(path);

    if !research_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Path not found",
        ));
    }

    if !research_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

    let mut found_files = Vec::new();
    for entry in std::fs::read_dir(research_path)? {
        let file = match entry {
            Ok(file) => file,
            Err(_) => continue,
        };

        let file_path = file.path();
        if file_path.is_dir() && criteria.recursive {
            if let Some(subdirectory_path) = file_path.to_str() {
                if let Ok(subdirectory_files) = find(subdirectory_path, criteria) {
                    found_files.extend(subdirectory_files);
                }
            }
        } else if file_path.is_file() && file_utils::accept(&file, criteria) {
            if let Some(file_str) = file_path.to_str() {
                found_files.push(file_str.to_string());
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
    fn empty_directory() {
        let criteria = FindCriteria::new();
        let result = find("tests_files/empty", &criteria);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn none_existing_path() {
        let criteria = FindCriteria::new();
        let result = find("tests_files/empty2", &criteria);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Path not found");
    }

    #[test]
    fn path_is_a_file() {
        let criteria = FindCriteria::new();
        let result = find("tests_files/file", &criteria);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Path is not a directory");
    }

    #[test]
    fn validate_file_size_min_greater_than_max() {
        let criteria = FindCriteria::new()
            .file_size_minimum(200)
            .file_size_maximum(100);
        let result = file_criteria::validate(&criteria);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Minimum file size cannot be greater than maximum file size"
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
