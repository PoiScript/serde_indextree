# `serde_indextree`

Serializing `indextree` structure.

## Version support

| `indextree` version | `serde_indextree` version |
|---------------------|---------------------------|
| 3.3.x               | 0.1.x                     |
| 4.0.x               | 0.2.x                     |

## Usage

`serde_indextree` provides two struct: `Node` for serializing
a node and its descendants, `SiblingNodes` for serializing a
node and its siblings in sequence.

```rust
use indextree::Arena;
use serde::Serialize;
use serde_indextree::Node;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct HtmlElement {
    tag: &'static str
}

// <html>
// <head>
//     <title></title>
// <head>
// <body>
//     <h1></h1>
//     <h2></h2>
// </body>
// </html>
let arena = &mut Arena::new();
let a = arena.new_node(HtmlElement { tag: "html" });
let b = arena.new_node(HtmlElement { tag: "head" });
a.append(b, arena);
let c = arena.new_node(HtmlElement { tag: "title" });
b.append(c, arena);
let d = arena.new_node(HtmlElement { tag: "body" });
a.append(d, arena);
let e = arena.new_node(HtmlElement { tag: "h1" });
d.append(e, arena);
let f = arena.new_node(HtmlElement { tag: "h2" });
d.append(f, arena);

println!("{}", to_string_pretty(&Node::new(a, arena)).unwrap());
// {
//   "tag": "html",
//   "children": [
//     {
//       "tag": "head",
//       "children": [
//         {
//           "tag": "title"
//         }
//       ]
//     },
//     {
//       "tag": "body",
//       "children": [
//         {
//           "tag": "h1"
//         },
//         {
//           "tag": "h2"
//         }
//       ]
//     }
//   ]
// }
```

## Customization

Unfortunately, `serde_indextree` doesn't come up with any customization.

If you want to rename field names or anything, just copy the entire code
(only 40+ lines) and modify it at your wish.

## License

MIT
