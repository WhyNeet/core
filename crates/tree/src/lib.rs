#[derive(Debug)]
pub enum TreeNode {
    Node {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<TreeNode>,
    },
    Text(String),
}
