mod file_criteria;
mod file_utils;

use file_criteria::FindCriteria;

pub fn find(path: &str, find_criteria: &FindCriteria) -> Result<Vec<String>, std::io::Error> {
    let validation_result = file_criteria::validate(find_criteria);

    if validation_result.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            validation_result.err().unwrap(),
        ));
    }

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

    let mut find_files = Vec::<String>::new();
    let files = std::fs::read_dir(research_path)?;

    for file in files {
        if file.is_err() {
            continue;
        }

        let file = file?;

        if file.path().is_dir() {
            if find_criteria.recursive {
                match file.path().to_str() {
                    Some(subdirectory_path) => {
                        let subdirectory_files = find(subdirectory_path, &find_criteria);
                        if subdirectory_files.is_ok() {
                            find_files.append(&mut subdirectory_files?);
                        }
                    }
                    None => continue,
                }
            }
        } else if file.path().is_file() {
            if file_utils::accept(&file, &find_criteria) {
                find_files.push(file.path().to_str().unwrap().to_string());
            }
        }
    }
    return Ok(find_files);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn empty_directory() {
        let find_criteria: FindCriteria = FindCriteria::new();
        let result = find("tests_files/empty", &find_criteria);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn none_existing_path() {
        let find_criteria: FindCriteria = FindCriteria::new();
        let result = find("tests_files/empty2", &find_criteria);
        assert!(result.is_err());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Path not found");
    }

    #[test]
    fn path_is_a_file() {
        let find_criteria: FindCriteria = FindCriteria::new();
        let result = find("tests_files/file", &find_criteria);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Path is not a directory");
    }

    #[test]
    fn validate_file_size_min_greater_than_max() {
        let find_criteria = FindCriteria::new()
            .file_size_minimum(200)
            .file_size_maximum(100);
        let result = file_criteria::validate(&find_criteria);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Minimum file size cannot be greater than maximum file size"
        );
    }

    #[test]
    fn all_files() {
        let find_criteria: FindCriteria = FindCriteria::new();
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }

    #[test]
    fn not_recursive_files() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria.recursive(false);
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn search_name() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria.file_name(OsString::from("f"));
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_png() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria.file_extension(OsString::from("png"));
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_size_min() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria.file_size_minimum(89534);
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn search_size_max() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria.file_size_maximum(89534);
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn search_multiple_criteria() {
        let mut find_criteria: FindCriteria = FindCriteria::new();
        find_criteria = find_criteria
            .file_name(OsString::from("image"))
            .file_extension(OsString::from("png"))
            .file_size_minimum(89)
            .file_size_maximum(100000);
        let result = find("tests_files", &find_criteria);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
