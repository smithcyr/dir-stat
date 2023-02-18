use std::collections::HashMap;

pub type NodeId = (u64, u64);

pub struct NodeInfo {
    pub path: String,

    // tuple of mount id and node id to uniquely identify a file
    pub id: NodeId,
}

pub struct SymLinkInfo {
    pub path: String,
}

pub struct FileInfo {
    // size of the directory
    // u64 has a max of ~2 exabytes so u128 should be enough :P
    pub size: u128,

    pub node: NodeInfo,
}

pub struct DirectoryInfo {
    pub node: NodeInfo,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct NodeResult {
    pub size: u128,
    pub node_type: NodeType,
}

pub struct DirectoryResult {
    pub files: Vec<FileInfo>,
    pub directories: Vec<DirectoryInfo>,
    pub sym_links: Vec<SymLinkInfo>,
}

pub type DirectoryScanResult = HashMap<String, NodeResult>;

#[derive(Debug)]
pub struct ScanResult {
    pub result: DirectoryScanResult,
    pub double_count: DirectoryScanResult,
}