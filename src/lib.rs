use indextree::{Arena, NodeId};
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

#[derive(Serialize)]
pub struct Node<'a, T: Serialize> {
    #[serde(flatten)]
    data: &'a T,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<SiblingNodes<'a, T>>,
}

impl<'a, T: Serialize> Node<'a, T> {
    pub fn new(id: NodeId, arena: &'a Arena<T>) -> Self {
        let node = &arena[id];
        Node {
            data: &node.data,
            children: node
                .first_child()
                .map(|first| SiblingNodes::new(first, arena)),
        }
    }
}

pub struct SiblingNodes<'a, T: Serialize> {
    first: NodeId,
    arena: &'a Arena<T>,
}

impl<'a, T: Serialize> SiblingNodes<'a, T> {
    pub fn new(id: NodeId, arena: &'a Arena<T>) -> Self {
        SiblingNodes { first: id, arena }
    }
}

impl<T: Serialize> Serialize for SiblingNodes<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for node in self.first.following_siblings(&self.arena) {
            seq.serialize_element(&Node::new(node, &self.arena))?;
        }
        seq.end()
    }
}
