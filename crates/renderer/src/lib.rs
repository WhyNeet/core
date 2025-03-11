use tree::TreeNode;

pub struct CliTreeRenderer;

impl CliTreeRenderer {
    pub fn render_tree(root: TreeNode, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        match root {
            TreeNode::Node {
                name,
                attributes,
                children,
            } => match name.as_str() {
                "div" | "p" | "body" | "html" => {
                    for node in children {
                        Self::render_tree(node, f)?;
                        write!(f, "\n")?;
                    }
                }
                name if name.chars().nth(0).map(|c| c == 'h').unwrap_or(false)
                    && name
                        .chars()
                        .nth(1)
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false) =>
                {
                    write!(f, "[heading] ")?;
                    for node in children {
                        Self::render_tree(node, f)?;
                        write!(f, "\n")?;
                    }
                }
                "a" => {
                    write!(
                        f,
                        "[link:{}] ",
                        attributes
                            .iter()
                            .find(|(name, _)| name == "href")
                            .map(|(_, href)| href.as_str())
                            .unwrap_or("")
                    )?;
                    for node in children {
                        Self::render_tree(node, f)?;
                        write!(f, "\n")?;
                    }
                }
                "head" => {
                    for node in children {
                        Self::render_tree(node, f)?;
                    }
                }
                "meta" | "style" => (),
                "title" => {
                    write!(f, "Page Title: ")?;
                    for node in children {
                        Self::render_tree(node, f)?;
                        write!(f, "\n")?;
                    }
                    write!(f, "----------\n")?;
                }
                "script" => (),
                other => todo!("{other}"),
            },
            TreeNode::Text(text) => write!(f, "{text}")?,
        };

        Ok(())
    }
}
