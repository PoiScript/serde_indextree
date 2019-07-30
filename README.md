Serializing indextree structure.

# Usage

`serde_indextree` provides two struct: `Node` for serializing
a node and its descendants, `SiblingNodes` for serializing a
node and its siblings in sequence.

```rust
use indextree::Arena;
use serde::Serialize;
use serde_indextree::Node;
use serde_json::to_string;

#[derive(Serialize)]
struct HtmlElement {
    tag: &'static str
}

// <html><head><title></title><head><body><h1></h1><h2></h2></body></html>
let arena = &mut Arena::new();
let a = arena.new_node(HtmlElement { tag: "html" });
let b = arena.new_node(HtmlElement { tag: "head" });
a.append(b, arena).unwrap();
let c = arena.new_node(HtmlElement { tag: "title" });
b.append(c, arena).unwrap();
let d = arena.new_node(HtmlElement { tag: "body" });
a.append(d, arena).unwrap();
let e = arena.new_node(HtmlElement { tag: "h1" });
d.append(e, arena).unwrap();
let f = arena.new_node(HtmlElement { tag: "h2" });
d.append(f, arena).unwrap();

assert_eq!(
    to_string(&Node::new(a, arena)).unwrap(),
    "{\
        \"tag\": \"html\",\
        \"children\": [\
            {\
                \"tag\": \"head\",\
                \"children\": [\
                    { \"tag\": \"title\" }\
                ]\
            }, {\
                \"tag\": \"body\",\
                \"children\": [\
                    { \"tag\": \"h1\" },\
                    { \"tag\": \"h2\" }\
                ]\
            }\
        ]\
    }"
);
```

# Customization

Unfortunately, `serde_indextree` doesn't come up with any customization.

If you want to rename field names or anything, just copy the entire code
(only 40+ lines) and modify it at your wish.

# License

MIT
