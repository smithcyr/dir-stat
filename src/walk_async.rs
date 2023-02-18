use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use crate::process_directory::{process_directory};
use crate::types::{NodeResult, NodeId, NodeType, ScanResult};

// TODO: need to determine if threaded directory walk is faster might actually be slower
const THREAD_COUNT: u8 = 3; 

pub fn process_dir_threaded(root_path_str: String) -> ScanResult {
    // number of currently processing nodes
    // (this value will be used in threads when waiting for completion of traversal)
    let current_running_nodes = Arc::new(RwLock::new(0));

    // set of processed inode ids
    // if a currently processed node already exists in this set then we skip it
    // this happens when processing hard links thus only the first encounter of
    // an inode is processed which could lead to inconsistency between runs
    let processed_inode_ids: Arc<RwLock<HashSet<NodeId>>> = Arc::new(RwLock::new(HashSet::new()));

    // queue of nodes to process
    let dir_queue: Arc<RwLock<VecDeque<String>>> = Arc::new(RwLock::new(VecDeque::new()));

    // result to be analized and displayed
    let scan: Arc<Mutex<ScanResult>> = Arc::new(Mutex::new(ScanResult {
        result: HashMap::new(),
        double_count: HashMap::new(),
    }));

    // recursive iteration of all files/directories starting with root directory

    // add the root resolved path (after navigating symlink) to the queue
    dir_queue
        .write()
        .expect("Failed to initialize work queue")
        .push_front(root_path_str);

    // start n threads of node process
    let mut children = vec![];

    for _ in 1..THREAD_COUNT {
        let current_running_nodes_arc = Arc::clone(&current_running_nodes);
        let dir_queue_arc = Arc::clone(&dir_queue);
        let processed_inode_ids_arc = Arc::clone(&processed_inode_ids);
        let scan_arc = Arc::clone(&scan);
        children.push(thread::spawn(move || {
            loop {
                let other_running_nodes = *current_running_nodes_arc.read().unwrap();
                let next_dir = dir_queue_arc.write().unwrap().pop_back();
                match next_dir {
                    None => {
                        if other_running_nodes == 0 {
                            // if the queue is empty and there are no other nodes currently running, then quit
                            break;
                        }
                    }
                    Some(directory_path) => {
                        let mut running_nodes = current_running_nodes_arc.write().unwrap();
                        *running_nodes += 1;

                        let directory_result = match process_directory(&directory_path) {
                            Ok(r) => r,
                            Err(_) => {
                                // TODO: handle access errors
                                continue;
                            }
                        };

                        let mut unprocessed_directories: VecDeque<String> = VecDeque::new();
                        for directory in directory_result.directories {
                            if !processed_inode_ids_arc
                                .read()
                                .unwrap()
                                .contains(&directory.node.id)
                            {
                                unprocessed_directories.push_back(directory.node.path);
                            }
                        }
                        dir_queue_arc
                            .write()
                            .unwrap()
                            .append(&mut unprocessed_directories);

                        let mut scan_write = scan_arc.lock().unwrap();
                        let mut directory_size: u128 = 0;
                        for file in directory_result.files {
                            if !processed_inode_ids_arc
                                .read()
                                .unwrap()
                                .contains(&file.node.id)
                            {
                                processed_inode_ids_arc
                                    .write()
                                    .unwrap()
                                    .insert(file.node.id);
                                scan_write.result.insert(
                                    file.node.path,
                                    NodeResult {
                                        size: file.size,
                                        node_type: NodeType::File,
                                    },
                                );
                                directory_size += file.size;
                            } else {
                                scan_write.double_count.insert(
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
                            match scan_write.result.get_mut(&ancestor) {
                                Some(node) => {
                                    node.size += directory_size;
                                }
                                None => {
                                    scan_write.result.insert(
                                        ancestor,
                                        NodeResult {
                                            size: directory_size,
                                            node_type: NodeType::Directory,
                                        },
                                    );
                                }
                            }
                        }

                        let mut running_nodes = current_running_nodes_arc.write().unwrap();
                        *running_nodes -= 1;
                    }
                }
            }
        }));
    }

    for child in children {
        // wait for all worker threads to end
        let _ = child.join().unwrap();
    }

    return Arc::try_unwrap(scan).unwrap().into_inner().unwrap();
}
