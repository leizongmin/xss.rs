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
}

pub fn new() -> XSS {
    XSS::new()
}
