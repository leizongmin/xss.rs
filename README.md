# xss-rust
Sanitize untrusted HTML (to prevent XSS) with a configuration specified by a Whitelist.

```rust
extern crate xss;
use xss::*;

let myxss = xss::new();

// custom white list
myxss.use_default_white_list();
myxss.set_allow_tag("a", ["href", "target"]);
myxss.set_allow_tag("p", []);

// optional function
myxss.set_on_tag(|pos: Position, tag: HTMLTag| None);
myxss.set_on_ignore_tag(|pos: Position, tag: HTMLTag| None);
myxss.set_on_tag_attr(|pos: Position, attr: HTMLTagAttribute| None);
myxss.set_on_ignore_tag_attr(|pos: Position, attr: HTMLTagAttribute| None);
```
