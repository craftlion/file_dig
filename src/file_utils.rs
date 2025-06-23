use crate::file_criteria::FindCriteria;
use std::ffi::OsString;

pub fn accept(file: &std::fs::DirEntry, criteria: &FindCriteria) -> bool {
    if let Some(file_name) = criteria.file_name.as_ref() {
        if !contains(&file.file_name(), file_name) {
            return false;
        }
    }

    if let Some(extension) = criteria.file_extension.as_ref() {
        if file.path().extension() != Some(extension) {
            return false;
        }
    }

    if let Some(size_min) = criteria.file_size_minimum {
        if let Ok(metadata) = file.metadata() {
            if metadata.len() < size_min {
                return false;
            }
        } else {
            return false;
        }
    }

    if let Some(size_max) = criteria.file_size_maximum {
        if let Ok(metadata) = file.metadata() {
            if metadata.len() > size_max {
                return false;
            }
        } else {
            return false;
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
