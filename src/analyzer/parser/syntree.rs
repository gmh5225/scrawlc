use crate::analyzer::parser::node::Node;

#[derive(Debug)]
pub struct SynTree {
    pub nodes: Vec<Node>,
}

impl SynTree {
    /// Returns a new syntax tree structure.
    ///
    /// # Examples
    /// ```
    /// let syntree = scrawlc::SynTree::new();
    ///
    /// assert_eq!(syntree.nodes.len(), 0);
    /// ```
    pub fn new() -> Self {
        SynTree { nodes: Vec::new() }
    }
}
