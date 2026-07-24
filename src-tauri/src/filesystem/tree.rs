

use crate::filesystem::node::FileNode;

pub struct TreeNode {
    pub node: FileNode, 
    pub children: Vec<TreeNode>
}

