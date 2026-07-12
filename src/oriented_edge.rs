use serde::{Deserialize, Serialize};

use crate::{EdgeBase, NodeId};

/// Structure d'une arête orientée entre deux nœuds.
///
/// Cette struct définit uniquement la forme structurelle de l'arête.
/// Le sens métier est ajouté par composition dans une relation concrète.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrientedEdge {
    struct_edge_base: EdgeBase,
    struct_source: NodeId,
    struct_target: NodeId,
}

impl OrientedEdge {
    pub fn new(struct_source: NodeId, struct_target: NodeId) -> Self {
        Self {
            struct_edge_base: EdgeBase::new(),
            struct_source,
            struct_target,
        }
    }

    pub const fn struct_edge_base(&self) -> &EdgeBase {
        &self.struct_edge_base
    }

    pub const fn struct_source(&self) -> &NodeId {
        &self.struct_source
    }

    pub const fn struct_target(&self) -> &NodeId {
        &self.struct_target
    }

    pub fn update_timestamp(&mut self) {
        self.struct_edge_base.update_timestamp();
    }
}
