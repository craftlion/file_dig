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
    pub fn new() -> Self {
        Self {
            recursive: true,
            file_name: None,
            file_extension: None,
            file_size_minimum: None,
            file_size_maximum: None,
        }
    }

    pub fn new_with_recursive(recursive: bool) -> Self {
        Self {
            recursive,
            file_name: None,
            file_extension: None,
            file_size_minimum: None,
            file_size_maximum: None,
        }
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn file_name(mut self, name: OsString) -> Self {
        self.file_name = Some(name);
        self
    }

    pub fn file_extension(mut self, extension: OsString) -> Self {
        self.file_extension = Some(extension);
        self
    }

    pub fn file_size_minimum(mut self, size_minimum: u64) -> Self {
        self.file_size_minimum = Some(size_minimum);
        self
    }

    pub fn file_size_maximum(mut self, size_maximum: u64) -> Self {
        self.file_size_maximum = Some(size_maximum);
        self
    }
}

pub fn validate(criteria: &FindCriteria) -> Result<(), String> {
    if let (Some(min), Some(max)) = (criteria.file_size_minimum, criteria.file_size_maximum) {
        if min > max {
            return Err("Minimum file size cannot be greater than maximum file size".to_string());
        }
    }
    Ok(())
}
