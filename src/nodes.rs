use std::{
    ffi::OsString,
    fs,
    path::PathBuf,

};

/// Represents the type of a file system node.
///
/// # Variants
///
/// * `File` - A regular file.
/// * `Symlink` - A symbolic link.
/// * `Directory` - A directory.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeType {
    File,
    Symlink,
    Directory,
}

/// Represents a node in the file system, such as a file, directory, or symlink.
///
/// # Fields
///
/// * `file_name` - The name of the file, directory, or symlink as an `OsString`.
/// * `file_type` - The type of the node, represented by the `NodeType` enum.
#[derive(Clone, PartialEq, Eq)]
pub(super) struct Node {
    pub file_name: OsString,
    pub file_type: NodeType,
}

impl Node {
    /// Constructs a `Node` instance from a specified file system path.
    ///
    /// This function examines the given path to determine if it corresponds to a file, symbolic link, or directory.
    /// It then creates and returns a `Node` instance encapsulating this information, including the node's name
    /// and type. If the path does not exist or the node name cannot be determined, the function returns `None`.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to a `Path` object representing the file system path from which to create a `Node`.
    ///
    /// # Returns
    ///
    /// * `Option<Node>` - Returns `Some(Node)` containing the node's name and type if the path exists and the node
    /// name can be determined. Returns `None` if the path does not exist or the node name cannot be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let path = Path::new("/path/to/file.txt");
    /// if let Some(node) = Node::from_path(path) {
    ///     println!("Node type: {:?}", node.file_type);
    ///     println!("Node name: {:?}", node.file_name);
    /// }
    /// ```
    pub fn from_path(file_path: PathBuf) -> Option<Node> {
        if !file_path.exists() {
            return None;
        }

        let file_name = match file_path.file_name() {
            Some(name) => name.to_os_string(),
            None => return None,
        };

        let file_type = if file_path.is_file() {
            NodeType::File
        } else if file_path.is_symlink() {
            NodeType::Symlink
        } else {
            NodeType::Directory
        };

        Some(Node {
            file_type,
            file_name,
        })
    }
}

/// Retrieves a list of `Node` objects representing each file or directory within the given path.
///
/// # Arguments
/// * `path_buf` - A `PathBuf` instance representing the directory you wish to query.
///
/// # Returns
/// * `Option<Vec<Node>>` - Returns `Some(Vec<Node>)` containing all the `Node` instances within the directory
/// specified by `path_buf`. If the path does not exist, returns `None`.
pub(super) fn get_node_list(path_buf: PathBuf) -> Option<Vec<Node>> {
    let mut ret_vec: Vec<Node> = vec![];

    if !path_buf.exists() {
        return None;
    }

    let entries = match fs::read_dir(path_buf) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if let Some(node) = Node::from_path(path) {
            ret_vec.push(node);
        }
    }

    Some(ret_vec)
}
