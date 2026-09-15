use std::collections::HashMap;

use super::NodeId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeStoreError {
    ParentNotFound(NodeId),
    ChildNotFound(NodeId),
    AlreadyHasParent(NodeId),
    AlreadyChild(NodeId),
    WouldCreateCycle {
        parent: NodeId,
        child: NodeId,
    },
}

#[derive(Default)]
struct NodeRelationships {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

pub struct NodeStore {
    relationships: HashMap<NodeId, NodeRelationships>,
}

impl NodeStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, node_id: NodeId) -> bool {
        self.relationships
            .insert(node_id, NodeRelationships::default())
            .is_none()
    }

    pub fn unregister(&mut self, node_id: NodeId) -> bool {
        let Some(relationship) = self.relationships.remove(&node_id) else {
            return false;
        };

        if let Some(parent_id) = relationship.parent {
            if let Some(parent) = self.relationships.get_mut(&parent_id) {
                parent.children.retain(|id| *id != node_id);
            }
        }

        for child_id in relationship.children {
            if let Some(child) = self.relationships.get_mut(&child_id) {
                child.parent = None;
            }
        }

        true
    }

    pub fn contains(&self, node_id: NodeId) -> bool {
        self.relationships.contains_key(&node_id)
    }

    pub fn append_child(
        &mut self,
        parent_id: NodeId,
        child_id: NodeId,
    ) -> Result<(), NodeStoreError> {
        self.require_parent(parent_id)?;
        self.require_child(child_id)?;

        if parent_id == child_id {
            return Err(NodeStoreError::WouldCreateCycle {
                parent: parent_id,
                child: child_id,
            });
        }

        if self
            .relationships
            .get(&child_id)
            .and_then(|relationship| relationship.parent)
            .is_some()
        {
            return Err(NodeStoreError::AlreadyHasParent(child_id));
        }

        if self
            .relationships
            .get(&parent_id)
            .is_some_and(|relationship| {
                relationship.children.contains(&child_id)
            })
        {
            return Err(NodeStoreError::AlreadyChild(child_id));
        }

        if self.is_descendant(child_id, parent_id) {
            return Err(NodeStoreError::WouldCreateCycle {
                parent: parent_id,
                child: child_id,
            });
        }

        self.relationships
            .get_mut(&parent_id)
            .expect("parent was validated")
            .children
            .push(child_id);

        self.relationships
            .get_mut(&child_id)
            .expect("child was validated")
            .parent = Some(parent_id);

        Ok(())
    }

    pub fn remove_child(
        &mut self,
        parent_id: NodeId,
        child_id: NodeId,
    ) -> Result<bool, NodeStoreError> {
        self.require_parent(parent_id)?;
        self.require_child(child_id)?;

        let Some(parent) = self.relationships.get_mut(&parent_id) else {
            unreachable!("parent was validated");
        };

        let Some(index) = parent.children.iter().position(|id| *id == child_id) else {
            return Ok(false);
        };

        parent.children.remove(index);

        if let Some(child) = self.relationships.get_mut(&child_id) {
            child.parent = None;
        }

        Ok(true)
    }

    pub fn parent(
        &self,
        node_id: NodeId,
    ) -> Result<Option<NodeId>, NodeStoreError> {
        self.require_node(node_id)?;

        Ok(self
            .relationships
            .get(&node_id)
            .expect("node was validated")
            .parent)
    }

    pub fn children(
        &self,
        node_id: NodeId,
    ) -> Result<&[NodeId], NodeStoreError> {
        self.require_node(node_id)?;

        Ok(&self
            .relationships
            .get(&node_id)
            .expect("node was validated")
            .children)
    }

    pub fn child_count(
        &self,
        node_id: NodeId,
    ) -> Result<usize, NodeStoreError> {
        Ok(self.children(node_id)?.len())
    }

    pub fn first_child(
        &self,
        node_id: NodeId,
    ) -> Result<Option<NodeId>, NodeStoreError> {
        Ok(self.children(node_id)?.first().copied())
    }

    pub fn last_child(
        &self,
        node_id: NodeId,
    ) -> Result<Option<NodeId>, NodeStoreError> {
        Ok(self.children(node_id)?.last().copied())
    }

    fn require_node(&self, node_id: NodeId) -> Result<(), NodeStoreError> {
        if self.contains(node_id) {
            Ok(())
        } else {
            Err(NodeStoreError::ChildNotFound(node_id))
        }
    }

    fn require_parent(
        &self,
        node_id: NodeId,
    ) -> Result<(), NodeStoreError> {
        if self.contains(node_id) {
            Ok(())
        } else {
            Err(NodeStoreError::ParentNotFound(node_id))
        }
    }

    fn require_child(
        &self,
        node_id: NodeId,
    ) -> Result<(), NodeStoreError> {
        if self.contains(node_id) {
            Ok(())
        } else {
            Err(NodeStoreError::ChildNotFound(node_id))
        }
    }

    fn is_descendant(
        &self,
        ancestor_id: NodeId,
        candidate_id: NodeId,
    ) -> bool {
        let mut current = Some(candidate_id);

        while let Some(node_id) = current {
            if node_id == ancestor_id {
                return true;
            }

            current = self
                .relationships
                .get(&node_id)
                .and_then(|relationship| relationship.parent);
        }

        false
    }
}

impl Default for NodeStore {
    fn default() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeStore, NodeStoreError};
    use crate::dom::{Node, NodeType};

    fn node() -> Node {
        Node::new(NodeType::Element, "div")
    }

    fn registered_store(count: usize) -> (NodeStore, Vec<crate::dom::NodeId>) {
        let mut store = NodeStore::new();
        let mut ids = Vec::with_capacity(count);

        for _ in 0..count {
            let node = node();
            let id = node.id();

            assert!(store.register(id));

            ids.push(id);
        }

        (store, ids)
    }

    #[test]
    fn creates_empty_store() {
        let store = NodeStore::new();

        let node = node();

        assert!(!store.contains(node.id()));
    }

    #[test]
    fn registers_node() {
        let mut store = NodeStore::new();
        let node = node();

        assert!(store.register(node.id()));
        assert!(store.contains(node.id()));
    }

    #[test]
    fn cannot_register_same_node_twice() {
        let mut store = NodeStore::new();
        let node = node();

        assert!(store.register(node.id()));
        assert!(!store.register(node.id()));
    }

    #[test]
    fn appends_child() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        store.append_child(parent, child).unwrap();

        assert_eq!(store.parent(child).unwrap(), Some(parent));
        assert_eq!(store.children(parent).unwrap(), &[child]);
    }

    #[test]
    fn preserves_child_order() {
        let (mut store, ids) = registered_store(3);
        let parent = ids[0];
        let first = ids[1];
        let second = ids[2];

        store.append_child(parent, first).unwrap();
        store.append_child(parent, second).unwrap();

        assert_eq!(
            store.children(parent).unwrap(),
            &[first, second]
        );
    }

    #[test]
    fn removes_child() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        store.append_child(parent, child).unwrap();

        assert!(store.remove_child(parent, child).unwrap());
        assert_eq!(store.parent(child).unwrap(), None);
        assert!(store.children(parent).unwrap().is_empty());
    }

    #[test]
    fn removing_non_child_returns_false() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        assert!(!store.remove_child(parent, child).unwrap());
    }

    #[test]
    fn rejects_duplicate_child() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        store.append_child(parent, child).unwrap();

        assert_eq!(
            store.append_child(parent, child),
            Err(NodeStoreError::AlreadyHasParent(child))
        );
    }

    #[test]
    fn rejects_node_with_existing_parent() {
        let (mut store, ids) = registered_store(3);
        let first_parent = ids[0];
        let second_parent = ids[1];
        let child = ids[2];

        store.append_child(first_parent, child).unwrap();

        assert_eq!(
            store.append_child(second_parent, child),
            Err(NodeStoreError::AlreadyHasParent(child))
        );
    }

    #[test]
    fn rejects_self_as_child() {
        let (mut store, ids) = registered_store(1);
        let node_id = ids[0];

        assert_eq!(
            store.append_child(node_id, node_id),
            Err(NodeStoreError::WouldCreateCycle {
                parent: node_id,
                child: node_id,
            })
        );
    }

    #[test]
    fn rejects_cycle() {
        let (mut store, ids) = registered_store(3);
        let first = ids[0];
        let second = ids[1];
        let third = ids[2];

        store.append_child(first, second).unwrap();
        store.append_child(second, third).unwrap();

        assert_eq!(
            store.append_child(third, first),
            Err(NodeStoreError::WouldCreateCycle {
                parent: third,
                child: first,
            })
        );
    }

    #[test]
    fn reports_child_count() {
        let (mut store, ids) = registered_store(3);
        let parent = ids[0];
        let first = ids[1];
        let second = ids[2];

        assert_eq!(store.child_count(parent).unwrap(), 0);

        store.append_child(parent, first).unwrap();
        assert_eq!(store.child_count(parent).unwrap(), 1);

        store.append_child(parent, second).unwrap();
        assert_eq!(store.child_count(parent).unwrap(), 2);
    }

    #[test]
    fn reports_first_and_last_child() {
        let (mut store, ids) = registered_store(3);
        let parent = ids[0];
        let first = ids[1];
        let last = ids[2];

        assert_eq!(store.first_child(parent).unwrap(), None);
        assert_eq!(store.last_child(parent).unwrap(), None);

        store.append_child(parent, first).unwrap();
        store.append_child(parent, last).unwrap();

        assert_eq!(store.first_child(parent).unwrap(), Some(first));
        assert_eq!(store.last_child(parent).unwrap(), Some(last));
    }

    #[test]
    fn unregisters_root_node() {
        let (mut store, ids) = registered_store(1);
        let node_id = ids[0];

        assert!(store.unregister(node_id));
        assert!(!store.contains(node_id));
    }

    #[test]
    fn unregistering_child_detaches_it() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        store.append_child(parent, child).unwrap();

        assert!(store.unregister(child));
        assert!(!store.contains(child));
        assert!(store.children(parent).unwrap().is_empty());
    }

    #[test]
    fn unregistering_parent_detaches_children() {
        let (mut store, ids) = registered_store(2);
        let parent = ids[0];
        let child = ids[1];

        store.append_child(parent, child).unwrap();

        assert!(store.unregister(parent));
        assert!(!store.contains(parent));
        assert_eq!(store.parent(child).unwrap(), None);
    }

    #[test]
    fn rejects_unknown_parent() {
        let mut store = NodeStore::new();
        let parent = node();
        let child = node();

        store.register(child.id());

        assert_eq!(
            store.append_child(parent.id(), child.id()),
            Err(NodeStoreError::ParentNotFound(parent.id()))
        );
    }

    #[test]
    fn rejects_unknown_child() {
        let mut store = NodeStore::new();
        let parent = node();
        let child = node();

        store.register(parent.id());

        assert_eq!(
            store.append_child(parent.id(), child.id()),
            Err(NodeStoreError::ChildNotFound(child.id()))
        );
    }
}