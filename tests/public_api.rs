//! Integration tests for the public `common_graph` API.
//!
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use common_graph::{EdgeBase, EdgeId, NodeBase, NodeId, OrientedEdge};
use serde_json::json;

fn assert_uuid_v7_text(string_value: &str) {
    assert_eq!(string_value.len(), 36);
    assert_eq!(string_value.as_bytes()[14], b'7');
}

#[test]
fn node_ids_are_unique_uuid_v7_values() {
    let struct_first_id = NodeId::new();
    let struct_second_id = NodeId::new();

    assert_ne!(struct_first_id, struct_second_id);
    assert_uuid_v7_text(&struct_first_id.to_string());
    assert_uuid_v7_text(&struct_second_id.to_string());
}

#[test]
fn edge_ids_are_unique_uuid_v7_values() {
    let struct_first_id = EdgeId::new();
    let struct_second_id = EdgeId::new();

    assert_ne!(struct_first_id, struct_second_id);
    assert_uuid_v7_text(&struct_first_id.to_string());
    assert_uuid_v7_text(&struct_second_id.to_string());
}

#[test]
fn node_id_supports_text_round_trip() {
    let struct_original_id = NodeId::new();
    let string_id = struct_original_id.to_string();

    let struct_restored_id =
        NodeId::from_str(&string_id).expect("NodeId should parse from its own display value");

    assert_eq!(struct_original_id, struct_restored_id);
}

#[test]
fn edge_id_supports_text_round_trip() {
    let struct_original_id = EdgeId::new();
    let string_id = struct_original_id.to_string();

    let struct_restored_id =
        EdgeId::from_str(&string_id).expect("EdgeId should parse from its own display value");

    assert_eq!(struct_original_id, struct_restored_id);
}

#[test]
fn node_id_rejects_non_uuid_v7_values() {
    let struct_result = NodeId::from_str("550e8400-e29b-41d4-a716-446655440000");

    assert!(struct_result.is_err());
}

#[test]
fn edge_id_rejects_non_uuid_v7_values() {
    let struct_result = EdgeId::from_str("550e8400-e29b-41d4-a716-446655440000");

    assert!(struct_result.is_err());
}

#[test]
fn node_id_supports_json_round_trip() {
    let struct_original_id = NodeId::new();

    let string_json =
        serde_json::to_string(&struct_original_id).expect("NodeId should serialize to JSON");

    let struct_restored_id: NodeId =
        serde_json::from_str(&string_json).expect("NodeId should deserialize from JSON");

    assert_eq!(struct_original_id, struct_restored_id);
}

#[test]
fn edge_id_supports_ron_round_trip() {
    let struct_original_id = EdgeId::new();

    let string_ron = ron::to_string(&struct_original_id).expect("EdgeId should serialize to RON");

    let struct_restored_id: EdgeId =
        ron::from_str(&string_ron).expect("EdgeId should deserialize from RON");

    assert_eq!(struct_original_id, struct_restored_id);
}

#[test]
fn node_base_creates_consistent_structural_data() {
    let struct_node_base = NodeBase::new();

    assert_eq!(
        struct_node_base.struct_created_at(),
        struct_node_base.struct_updated_at()
    );

    assert_uuid_v7_text(&struct_node_base.struct_id().to_string());
}

#[test]
fn node_base_updates_its_timestamp() {
    let mut struct_node_base = NodeBase::new();
    let struct_previous_timestamp = *struct_node_base.struct_updated_at();

    thread::sleep(Duration::from_millis(1));
    struct_node_base.update_timestamp();

    assert!(struct_node_base.struct_updated_at() >= &struct_previous_timestamp);
}

#[test]
fn edge_base_creates_consistent_structural_data() {
    let struct_edge_base = EdgeBase::new();

    assert_eq!(
        struct_edge_base.struct_created_at(),
        struct_edge_base.struct_updated_at()
    );

    assert_uuid_v7_text(&struct_edge_base.struct_id().to_string());
}

#[test]
fn edge_base_updates_its_timestamp() {
    let mut struct_edge_base = EdgeBase::new();
    let struct_previous_timestamp = *struct_edge_base.struct_updated_at();

    thread::sleep(Duration::from_millis(1));
    struct_edge_base.update_timestamp();

    assert!(struct_edge_base.struct_updated_at() >= &struct_previous_timestamp);
}

#[test]
fn node_base_rejects_an_update_before_creation() {
    let struct_node_id = NodeId::new();

    let struct_value = json!({
        "struct_id": struct_node_id.to_string(),
        "struct_created_at": "2026-07-13T12:00:00Z",
        "struct_updated_at": "2026-07-13T11:59:59Z"
    });

    let struct_result = serde_json::from_value::<NodeBase>(struct_value);

    assert!(struct_result.is_err());
}

#[test]
fn edge_base_rejects_an_update_before_creation() {
    let struct_edge_id = EdgeId::new();

    let struct_value = json!({
        "struct_id": struct_edge_id.to_string(),
        "struct_created_at": "2026-07-13T12:00:00Z",
        "struct_updated_at": "2026-07-13T11:59:59Z"
    });

    let struct_result = serde_json::from_value::<EdgeBase>(struct_value);

    assert!(struct_result.is_err());
}

#[test]
fn oriented_edge_preserves_source_and_target() {
    let struct_source = NodeId::new();
    let struct_target = NodeId::new();

    let struct_edge = OrientedEdge::new(struct_source, struct_target);

    assert_eq!(struct_edge.struct_source(), &struct_source);
    assert_eq!(struct_edge.struct_target(), &struct_target);
}

#[test]
fn oriented_edge_allows_self_loops() {
    let struct_node_id = NodeId::new();
    let struct_edge = OrientedEdge::new(struct_node_id, struct_node_id);

    assert_eq!(struct_edge.struct_source(), struct_edge.struct_target());
}

#[test]
fn oriented_edge_supports_json_round_trip() {
    let struct_source = NodeId::new();
    let struct_target = NodeId::new();
    let struct_original_edge = OrientedEdge::new(struct_source, struct_target);

    let string_json = serde_json::to_string(&struct_original_edge)
        .expect("OrientedEdge should serialize to JSON");

    let struct_restored_edge: OrientedEdge =
        serde_json::from_str(&string_json).expect("OrientedEdge should deserialize from JSON");

    assert_eq!(struct_original_edge, struct_restored_edge);
}

#[test]
fn oriented_edge_updates_its_base_timestamp() {
    let mut struct_edge = OrientedEdge::new(NodeId::new(), NodeId::new());

    let struct_previous_timestamp = *struct_edge.struct_edge_base().struct_updated_at();

    thread::sleep(Duration::from_millis(1));
    struct_edge.update_timestamp();

    assert!(struct_edge.struct_edge_base().struct_updated_at() >= &struct_previous_timestamp);
}
