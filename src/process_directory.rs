use std::os::macos::fs::MetadataExt;
use std::fs;

use crate::types::{DirectoryInfo, DirectoryResult, FileInfo, NodeInfo, SymLinkInfo};

pub fn process_directory(directory_path: &String) -> std::io::Result<DirectoryResult> {
    let mut directories: Vec<DirectoryInfo> = Vec::new();
    let mut files: Vec<FileInfo> = Vec::new();
    let mut sym_links: Vec<SymLinkInfo> = Vec::new();
    for file in fs::read_dir(directory_path)? {
        let path = file.unwrap().path();
        let path_str = path.to_str().unwrap();
        let sym_meta = fs::symlink_metadata(path_str).unwrap();

        // if the node is a symlink then ignore it
        if sym_meta.file_type().is_symlink() {
            sym_links.push(SymLinkInfo {
                path: String::from(path_str)
            });
            continue;
        }
        let metadata = fs::metadata(path_str).unwrap();
        let node = NodeInfo {
            id: (metadata.st_dev(), metadata.st_ino()),
            path: String::from(path_str),
        };

        // if the node is a file then add a NodeInfo under its path to the result with file size
        if sym_meta.is_file() {
            files.push(FileInfo {
                size: sym_meta.st_size().into(),
                node,
            });
        } else if sym_meta.is_dir() {
            directories.push(DirectoryInfo { node });
        }
    }
    Ok(DirectoryResult { files, directories, sym_links })
}