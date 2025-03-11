use anstyle::{Color, RgbColor, Style};
use terminal_link::Link;
use tree::TreeNode;

#[derive(Debug, Clone, Copy)]
pub enum TextStyling {
    Bold,
    Italic,
    Underline,
    Link,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    enclosing: Option<Box<TextStyle>>,
    styling: TextStyling,
}

impl TextStyle {
    pub fn get_style(&self) -> Style {
        let mut style = Style::new();

        let mut current = Some(self);

        while let Some(styling) = current {
            style = match styling.styling {
                TextStyling::Bold => style.bold(),
                TextStyling::Italic => style.italic(),
                TextStyling::Underline => style.underline(),
                TextStyling::Link => style
                    .underline()
                    .underline_color(Some(Color::Rgb(RgbColor(0, 0, 255)))),
            };

            current = styling.enclosing.as_ref().map(|s| s.as_ref());
        }

        style
    }
}

pub struct CliTreeRenderer {
    current_style: Option<Box<TextStyle>>,
}

impl CliTreeRenderer {
    pub fn new() -> Self {
        Self {
            current_style: None,
        }
    }

    pub fn render_tree(
        &mut self,
        root: TreeNode,
        f: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        match root {
            TreeNode::Node {
                name,
                attributes,
                children,
            } => match name.as_str() {
                "div" | "p" | "body" | "html" | "img" | "ul" => {
                    for node in children {
                        self.render_tree(node, f)?;
                    }
                    write!(f, "\n")?;
                }
                "small" => {
                    for node in children {
                        self.render_tree(node, f)?;
                    }
                }
                "i" => {
                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Italic,
                    }));

                    for node in children {
                        self.render_tree(node, f)?;
                    }

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;
                }
                "b" => {
                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Bold,
                    }));

                    for node in children {
                        self.render_tree(node, f)?;
                    }

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;
                }
                "u" => {
                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Underline,
                    }));

                    for node in children {
                        self.render_tree(node, f)?;
                    }

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;
                }
                "li" => {
                    write!(f, "  - ")?;
                    for node in children {
                        self.render_tree(node, f)?;
                    }
                    write!(f, "\n")?;
                }
                name if name.chars().nth(0).map(|c| c == 'h').unwrap_or(false)
                    && name
                        .chars()
                        .nth(1)
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false) =>
                {
                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Bold,
                    }));

                    for node in children {
                        self.render_tree(node, f)?;
                    }
                    write!(f, "\n")?;

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;
                }
                "a" => {
                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Link,
                    }));

                    let href = attributes
                        .iter()
                        .find(|(name, _)| name == "href")
                        .map(|(_, href)| href.as_str())
                        .unwrap_or("");

                    let mut buf = vec![];
                    for node in children {
                        self.render_tree(node, &mut buf)?;
                    }

                    let contents = String::from_utf8_lossy(&buf).into_owned();

                    write!(f, "{}", Link::new(&contents, href))?;

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;
                }
                "head" => {
                    for node in children {
                        self.render_tree(node, f)?;
                    }
                }
                "meta" | "style" | "link" => (),
                "br" => write!(f, "\n")?,
                "hr" => write!(f, "\n-------------------------\n")?,
                "title" => {
                    write!(f, "\n")?;

                    let enclosing = self.current_style.take();
                    self.current_style.replace(Box::new(TextStyle {
                        enclosing,
                        styling: TextStyling::Bold,
                    }));

                    for node in children {
                        self.render_tree(node, f)?;
                    }

                    let enclosing_style = self.current_style.take().unwrap().enclosing;
                    self.current_style = enclosing_style;

                    write!(f, "\n----------\n")?;
                }
                "script" => (),
                other => todo!("{other}"),
            },
            TreeNode::Text(text) => {
                let style = self
                    .current_style
                    .as_ref()
                    .map(|style| style.get_style())
                    .unwrap_or(Style::new());
                write!(f, "{style}{text}{style:#}")?;
            }
        };

        Ok(())
    }
}
