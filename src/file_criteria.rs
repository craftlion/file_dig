use std::ffi::OsString;

#[derive(Debug)]
pub struct FindCriteria {
    pub recursive: bool,
    pub file_name: Option<OsString>,
    pub file_extension: Option<OsString>,
    pub file_size_minimum: Option<u64>,
    pub file_size_maximum: Option<u64>,
}

impl FindCriteria {
    pub fn new() -> FindCriteria {
        FindCriteria {
            recursive: true,
            file_name: None,
            file_extension: None,
            file_size_minimum: None,
            file_size_maximum: None,
        }
    }

    pub fn new_with_recursive(recursive: bool) -> FindCriteria {
        FindCriteria {
            recursive,
            file_name: None,
            file_extension: None,
            file_size_minimum: None,
            file_size_maximum: None,
        }
    }

    pub fn recursive(mut self, recursive: bool) -> FindCriteria {
        self.recursive = recursive;
        self
    }

    pub fn file_name(mut self, name: OsString) -> FindCriteria {
        self.file_name = Some(name);
        self
    }

    pub fn file_extension(mut self, extension: OsString) -> FindCriteria {
        self.file_extension = Some(extension);
        self
    }

    pub fn file_size_minimum(mut self, size_minimum: u64) -> FindCriteria {
        self.file_size_minimum = Some(size_minimum);
        self
    }

    pub fn file_size_maximum(mut self, size_maximum: u64) -> FindCriteria {
        self.file_size_maximum = Some(size_maximum);
        self
    }
}

pub fn validate(search_criteria: &FindCriteria) -> Result<(), String> {
    if search_criteria.file_size_minimum.is_some() && search_criteria.file_size_maximum.is_some() {
        if search_criteria.file_size_minimum.unwrap() > search_criteria.file_size_maximum.unwrap() {
            return Err("Minimum file size cannot be greater than maximum file size".to_string());
        }
    }

    Ok(())
}
