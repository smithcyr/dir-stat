use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use crate::process_directory::{process_directory};
use crate::types::{NodeId, NodeResult, NodeType};

type ScanResult = HashMap<String, NodeResult>;

pub fn process_dir(root_path_str: String) -> ScanResult {
    // set of processed inode ids
    // if a currently processed node already exists in this set then we skip it
    // this happens when processing hard links thus only the first encounter of
    // an inode is processed which could lead to inconsistency between runs
    let mut processed_inode_ids: HashSet<NodeId> = HashSet::new();
    let stop_path = root_path_str.clone();

    // queue of nodes to process
    let mut dir_queue: VecDeque<String> = VecDeque::new();
    dir_queue.push_front(root_path_str);
    

    // result to be analized and displayed
    let mut result: ScanResult = HashMap::new();

    loop {
        let next_dir = dir_queue.pop_front();
        match next_dir {
            None => break,
            Some(directory_path) => {
                let directory_result = process_directory(&directory_path);

                let mut unprocessed_directories: VecDeque<String> = VecDeque::new();
                for directory in directory_result.directories {
                    if !processed_inode_ids.contains(&directory.node.id) {
                        unprocessed_directories.push_back(directory.node.path);
                    }
                }
                dir_queue.append(&mut unprocessed_directories);

                let mut directory_size: u128 = 0;
                for file in directory_result.files {
                    // prevent multiple counting hard linked files
                    if !processed_inode_ids.contains(&file.node.id) {
                        processed_inode_ids.insert(file.node.id);
                        directory_size += file.size;
                        result.insert(
                            file.node.path,
                            NodeResult {
                                size: file.size,
                                node_type: NodeType::File,
                            },
                        );
                    }
                }

                // add directory size to all parent directories
                for ancestor in Path::new(&directory_path)
                    .ancestors()
                    .map(|ancestor| String::from(ancestor.to_str().unwrap()))
                {
                    match result.get_mut(&ancestor) {
                        Some(node) => {
                            node.size += directory_size;
                        }
                        None => {
                            result.insert(
                                ancestor.clone(),
                                NodeResult {
                                    size: directory_size,
                                    node_type: NodeType::Directory,
                                },
                            );
                        }
                    }

                    // stop walking ancestors if we have reached the starting directory
                    if stop_path.eq(&ancestor) {
                        break;
                    }
                }
            }
        }
    }

    result
}