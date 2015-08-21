extern crate htmlstream;

use std::collections::HashMap;
use htmlstream::*;


pub struct XSS {
    on_tag: Option<Box<FnMut(Position, HTMLTag) -> Option<String>>>,
    on_ignore_tag: Option<Box<FnMut(Position, HTMLTag) -> Option<String>>>,
    on_tag_attr: Option<Box<FnMut(Position, HTMLTagAttribute) -> Option<String>>>,
    on_ignore_tag_attr: Option<Box<FnMut(Position, HTMLTagAttribute) -> Option<String>>>,
    allowed_tags: HashMap<String, String>,
}

impl XSS {
    pub fn new() -> XSS {
        XSS {
            on_tag: None,
            on_ignore_tag: None,
            on_tag_attr: None,
            on_ignore_tag_attr: None,
            allowed_tags: HashMap::new(),
        }
    }

    pub fn set_on_tag<F>(&mut self, f: F) where
    F: FnMut(Position, HTMLTag) -> Option<String> + 'static {
        self.on_tag = Some(Box::new(f));
    }

    pub fn set_on_ignore_tag<F>(&mut self, f: F) where
    F: FnMut(Position, HTMLTag) -> Option<String> + 'static {
        self.on_ignore_tag = Some(Box::new(f));
    }

    pub fn set_on_tag_attr<F>(&mut self, f: F) where
    F: FnMut(Position, HTMLTagAttribute) -> Option<String> + 'static {
        self.on_tag_attr = Some(Box::new(f));
    }

    pub fn set_on_ignore_tag_attr<F>(&mut self, f: F) where
    F: FnMut(Position, HTMLTagAttribute) -> Option<String> + 'static {
        self.on_ignore_tag_attr = Some(Box::new(f));
    }

    pub fn set_allow_tag(&mut self, name: &str, attrs: Vec<&str>) {
        for attr in attrs {
            self.allowed_tags.insert(name.to_string(), attr.to_string());
        }
    }

    pub fn use_default_white_list(&mut self) {
        self.set_allow_tag("a", vec!["href", "title"]);
    }

    fn escape(&self, html: &str) -> String {
        String::from(html).replace("<", "&lt;").replace(">", "&gt;")
    }

    pub fn sanitize(&self, html: &str) -> String {
        let mut ret_html = "".to_string();
        for (_, tag) in tag_iter(html) {
            println!("{:?}", tag);
            match tag.state {
                HTMLTagState::Text => ret_html.push_str(&tag.html),
                HTMLTagState::Closing | HTMLTagState::Opening | HTMLTagState::SelfClosing => {
                    match self.allowed_tags.get(&tag.name) {
                        None => ret_html.push_str(&(self.escape(&tag.html))),
                        Some(allowed_attrs) => {
                            println!("{:?}", allowed_attrs);
                            let mut attrs = "".to_string();
                            for (_, attr) in attr_iter(&tag.attributes) {
                                println!("{:?}", attr);
                                if allowed_attrs.contains(&attr.name) {
                                    println!("{:?} contains {:?}", allowed_attrs, attr.name);
                                    if attr.value.is_empty() {
                                        attrs.push_str(&attr.name);
                                    } else {
                                        attrs.push_str(&attr.name);
                                        attrs.push_str("=\"");
                                        attrs.push_str(&attr.value);
                                        attrs.push_str("\" ");
                                    }
                                }
                            }
                            match tag.state {
                                HTMLTagState::Opening => {
                                    ret_html.push_str("<");
                                    ret_html.push_str(&tag.name);
                                    ret_html.push_str(" ");
                                    ret_html.push_str(&attrs);
                                    ret_html.push_str(">");
                                },
                                HTMLTagState::Closing => {
                                    ret_html.push_str("</");
                                    ret_html.push_str(&tag.name);
                                    ret_html.push_str(">");
                                },
                                HTMLTagState::SelfClosing => {
                                    ret_html.push_str("<");
                                    ret_html.push_str(&tag.name);
                                    ret_html.push_str(" ");
                                    ret_html.push_str(&attrs);
                                    ret_html.push_str("/>");
                                },
                                _ => {}
                            }
                        },
                    }
                },
            }
        }
        return ret_html;
    }
}

pub fn new() -> XSS {
    XSS::new()
}
