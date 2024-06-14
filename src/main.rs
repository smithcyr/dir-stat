pub mod utils;

use std::fs;
use std::time::Instant;
use std::path::MAIN_SEPARATOR;
use structopt::StructOpt;

use dir_stat::walk::process_dir;
use dir_stat::walk_async::process_dir_threaded;
use dir_stat::types::NodeType;
use crate::utils::to_decimal_prefix;

#[derive(Debug, StructOpt)]
#[structopt(name = "dir-stat", about = "File and directory size analysis")]
struct Opt {
    /// Path of the directory to start at.
    path: String,

    /// Number of largest files to list.
    #[structopt(long, default_value = "30")]
    top: u8,

    /// show total size of multiple-referenced files
    #[structopt(long)]
    hardlinks: bool,

    /// number of threads to use
    #[structopt(long, default_value = "1")]
    threads: u8
}

fn main() -> Result<(), String> {
    let start = Instant::now();
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
    let scan = if opt.threads > 1 {
        process_dir_threaded(root_path_str, opt.threads)
    } else {
        process_dir(root_path_str)
    };

    let mut entries: Vec<(&_, &_)> = scan.result.iter().collect();
    entries.sort_by(|a, b| {
        if a.1.node_type != b.1.node_type {
            if a.1.node_type == NodeType::File {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
        b.1.size.cmp(&a.1.size)
    });

    println!("Runtime: {duration:.2?}", duration=start.elapsed());

    let mut count = 0;
    for entry in entries {
        if entry.1.node_type == NodeType::File {
            count += 1;
            if count > opt.top {
                break;
            }
            println!("{path}{dir} {size}", path = entry.0, size = to_decimal_prefix(entry.1.size as i128), dir = if entry.1.node_type == NodeType::Directory { MAIN_SEPARATOR.into() } else { String::new() });
        }
    }

    if opt.hardlinks {
        let double_count: u128 = scan.double_count.iter().fold(0, |acc, entry| {
            acc + entry.1.size
        });
        println!("\nFiles referenced multiple times (via hardlink): {}", to_decimal_prefix(double_count as i128));
    }

    return Result::Ok(());
}
