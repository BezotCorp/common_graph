//! Fondations communes et réutilisables des graphes BezotCorp.
//!
//! Ce crate contient uniquement les types structurels communs.
//! Il ne contient aucune connaissance métier liée aux modèles IA,
//! aux providers, au dataset ou au routage.

mod base;
mod id;
mod oriented_edge;

pub use base::{EdgeBase, NodeBase};
pub use id::{EdgeId, GraphIdParseError, NodeId};
pub use oriented_edge::OrientedEdge;
