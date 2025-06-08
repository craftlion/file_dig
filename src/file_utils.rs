use crate::file_criteria::FindCriteria;
use std::ffi::OsString;

pub fn accept(_file: &std::fs::DirEntry, search_criteria: &FindCriteria) -> bool {
    if search_criteria.file_name.is_some() {
        if !contains(
            &_file.file_name(),
            &(search_criteria.file_name.clone().unwrap()),
        ) {
            return false;
        }
    }
    if search_criteria.file_extension.is_some() {
        if let Some(searched_extension) = &search_criteria.file_extension {
            if _file.path().extension().is_none()
                || _file.path().extension().unwrap() != searched_extension
            {
                return false;
            }
        }
    }
    /*
    if search_criteria.file_type.is_some() {
        if let Some(file_type) = &search_criteria.file_type {
            if !_file.path().is_file()
                || _file
                    .path()
                    .extension()
                    .map_or(false, |ext| ext != file_type)
            {
                return false;
            }
        }
    }
    */
    if search_criteria.file_size_minimum.is_some() {
        if let Some(size_minimum) = search_criteria.file_size_minimum {
            let metadata = _file.metadata();
            if metadata.is_err() || metadata.as_ref().unwrap().len() < size_minimum {
                return false;
            }
        }
    }
    if search_criteria.file_size_maximum.is_some() {
        if let Some(size_maximum) = search_criteria.file_size_maximum {
            let metadata = _file.metadata();
            if metadata.is_err() || metadata.as_ref().unwrap().len() > size_maximum {
                return false;
            }
        }
    }

    true
}

fn contains(outer: &OsString, inner: &OsString) -> bool {
    let outer_bytes = outer.as_encoded_bytes();
    let inner_bytes = inner.as_encoded_bytes();

    outer_bytes
        .windows(inner_bytes.len())
        .any(|window| window == inner_bytes)
}
