#![allow(clippy::struct_field_names, clippy::must_use_candidate)]

//! Common and reusable foundations for `BezotCorp` graphs.
//!
//! This crate contains only shared structural graph types.
//! It contains no business knowledge related to AI models,
//! providers, datasets, or routing.

mod base;
mod id;
mod oriented_edge;

pub use base::{EdgeBase, NodeBase};
pub use id::{EdgeId, GraphIdParseError, NodeId};
pub use oriented_edge::OrientedEdge;
