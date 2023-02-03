use std::fs;
use structopt::StructOpt;

use dir_stat::walk::process_dir;
use dir_stat::types::NodeType;

#[derive(Debug, StructOpt)]
#[structopt(name = "dir-stat", about = "File size analysis")]
struct Opt {
    /// Path of the directory to start at.
    path: String,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    let path = opt.path;
    let canonicalized_path = fs::canonicalize(path).expect("Failed to canonalize path");
    let root_path_str = String::from(canonicalized_path.to_str().unwrap());
    let path_metadata = fs::metadata(canonicalized_path).expect("Failed to access path metadata");
    // follow symlink to root path
    if !path_metadata.is_dir() {
        // do not proceed if targetting a non-directory
        return Result::Err(String::from("Root path is not a directory."));
    }

    let result = process_dir(root_path_str);
    let mut entries: Vec<(&_, &_)> = result.iter().collect();
    entries.sort_by(|a, b| {
        if a.1.node_type != b.1.node_type {
            if a.1.node_type == NodeType::File {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
        a.1.size.cmp(&b.1.size)
    });

    for entry in entries {
        println!("{path} {size}", path = entry.0, size = entry.1.size);
    }
    return Result::Ok(());
}
