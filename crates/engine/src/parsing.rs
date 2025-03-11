use std::{cell::RefCell, rc::Rc};

use lol_html::{HtmlRewriter, Settings, doc_text, element, html_content::Element};
use tree::TreeNode;

use crate::util;

pub struct AstBuilder {
    root: Vec<TreeNode>,
    stack: Vec<TreeNode>,
}

impl AstBuilder {
    pub fn new() -> Self {
        Self {
            root: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn node_start(&mut self, node: TreeNode) {
        self.stack.push(node);
    }

    pub fn node_end(&mut self) {
        let node = self.stack.pop().unwrap();
        if let Some(TreeNode::Node { children, .. }) = self.stack.last_mut() {
            children.push(node);
        } else {
            self.root.push(node);
        }
    }

    pub fn push_node(&mut self, node: TreeNode) {
        if let Some(TreeNode::Node { children, .. }) = self.stack.last_mut() {
            children.push(node);
        } else {
            self.root.push(node);
        }
    }
}

pub struct DocumentParser<'a> {
    input: &'a str,
}

impl<'a> DocumentParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse_document(&mut self) -> Vec<TreeNode> {
        let builder = Rc::new(RefCell::new(AstBuilder::new()));

        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("*", |el: &mut Element| {
                    let attributes = el
                        .attributes()
                        .iter()
                        .map(|attr| (attr.name(), attr.value()))
                        .collect();

                    let node = TreeNode::Node {
                        name: el.tag_name(),
                        attributes,
                        children: vec![],
                    };

                    builder.borrow_mut().node_start(node);

                    if let Some(handlers) = el.end_tag_handlers() {
                        let builder = Rc::clone(&builder);
                        handlers.push(Box::new(move |_| {
                            builder.borrow_mut().node_end();
                            Ok(())
                        }));
                    } else {
                        builder.borrow_mut().node_end();
                    }

                    Ok(())
                })],
                document_content_handlers: vec![doc_text!(|t| {
                    let text = util::strip_text_indent(t.as_str());
                    if text.len() > 0 {
                        builder.borrow_mut().push_node(TreeNode::Text(text));
                    }
                    Ok(())
                })],
                ..Default::default()
            },
            |c: &[u8]| output.extend_from_slice(c), // No output needed since we're building an AST
        );

        rewriter.write(self.input.as_bytes()).unwrap();

        builder.replace(AstBuilder::new()).root
    }
}
