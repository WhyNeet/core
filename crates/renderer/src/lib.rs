use tree::TreeNode;

pub struct CliTreeRenderer;

impl CliTreeRenderer {
    pub fn render_tree(root: TreeNode, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // match root {
        //     TreeNode::DocType(_) => (),
        //     TreeNode::Html(nodes) | TreeNode::Body(nodes) => {
        //         for node in nodes {
        //             Self::render_tree(node, f)?;
        //         }
        //     }
        //     TreeNode::Paragraph(nodes) => {
        //         for node in nodes {
        //             Self::render_tree(node, f)?;
        //         }

        //         write!(f, "\n")?;
        //     }
        //     TreeNode::Text(text) => {
        //         write!(f, "{text}")?;
        //     }
        //     TreeNode::Link(nodes) => {
        //         write!(f, "[link] ")?;

        //         for node in nodes {
        //             Self::render_tree(node, f)?;
        //         }
        //     }
        //     TreeNode::Heading(nodes) => {
        //         write!(f, "[heading] ")?;

        //         for node in nodes {
        //             Self::render_tree(node, f)?;
        //         }

        //         write!(f, "\n")?;
        //     }
        //     _ => (),
        // };

        Ok(())
    }
}
