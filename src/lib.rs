#![crate_name = "xss"]
#![doc(html_root_url = "https://leizongmin.github.io/xss.rs")]

extern crate htmlstream;

pub use htmlstream::Position;
pub use htmlstream::HTMLTag;
pub use htmlstream::HTMLTagAttribute;
pub use htmlstream::HTMLTagState;

mod xss;
pub use xss::*;
