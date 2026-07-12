use serde::{Deserialize, Serialize};

use crate::{EdgeBase, NodeId};

/// Structural representation of a directed edge between two nodes.
///
/// This type defines only the structural shape of the edge.
/// Business semantics are added through composition in a concrete relation type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrientedEdge {
    struct_edge_base: EdgeBase,
    struct_source: NodeId,
    struct_target: NodeId,
}

impl OrientedEdge {
    /// Creates an oriented edge between a source node and a target node.
    pub fn new(struct_source: NodeId, struct_target: NodeId) -> Self {
        Self {
            struct_edge_base: EdgeBase::new(),
            struct_source,
            struct_target,
        }
    }

    /// Returns the common structural data of the edge.
    pub const fn struct_edge_base(&self) -> &EdgeBase {
        &self.struct_edge_base
    }

    /// Returns the source node identifier.
    pub const fn struct_source(&self) -> &NodeId {
        &self.struct_source
    }

    /// Returns the target node identifier.
    pub const fn struct_target(&self) -> &NodeId {
        &self.struct_target
    }

    /// Updates the edge modification timestamp.
    pub fn update_timestamp(&mut self) {
        self.struct_edge_base.update_timestamp();
    }
}
