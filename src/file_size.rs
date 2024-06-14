use std::fs::Metadata;

#[cfg(unix)]
mod implementation {
    use super::*;

    use std::os::unix::fs::MetadataExt;

    pub fn file_size(metadata: &Metadata) -> u128 {
        u128::from(metadata.blocks()) * 512
    }
}

#[cfg(windows)]
mod implementation {
    use super::*;

    use std::os::windows::fs::MetadataExt;

    pub fn file_size(metadata: &Metadata) -> u128 {
        u128::from(metadata.size())
    }
}

#[cfg(not(any(windows, unix)))]
mod implementation {
    use super::*;

    pub fn file_size(metadata: &Metadata) -> u128 {
        u128::from(metadata.len())
    }
}

pub fn file_size(metadata: &Metadata) -> u128 {
    self::implementation::file_size(metadata)
}
