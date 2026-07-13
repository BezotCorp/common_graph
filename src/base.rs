use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{EdgeId, NodeId};

/// Common structural data shared by all nodes.
///
/// Concrete node types extend this data through composition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NodeBase {
    struct_id: NodeId,
    struct_created_at: DateTime<Utc>,
    struct_updated_at: DateTime<Utc>,
}

impl NodeBase {
    /// Creates a node base with a fresh identifier and current timestamps.
    pub fn new() -> Self {
        let struct_now = Utc::now();

        Self {
            struct_id: NodeId::new(),
            struct_created_at: struct_now,
            struct_updated_at: struct_now,
        }
    }

    /// Returns the node identifier.
    pub const fn struct_id(&self) -> &NodeId {
        &self.struct_id
    }

    /// Returns the node creation timestamp.
    pub const fn struct_created_at(&self) -> &DateTime<Utc> {
        &self.struct_created_at
    }

    /// Returns the node update timestamp.
    pub const fn struct_updated_at(&self) -> &DateTime<Utc> {
        &self.struct_updated_at
    }

    /// Updates the node modification timestamp without allowing it to regress.
    pub fn update_timestamp(&mut self) {
        let struct_now = Utc::now();

        if struct_now > self.struct_updated_at {
            self.struct_updated_at = struct_now;
        }
    }
}

impl Default for NodeBase {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NodeBase {
    fn deserialize<DeserializerType>(
        deserializer: DeserializerType,
    ) -> Result<Self, DeserializerType::Error>
    where
        DeserializerType: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NodeBaseData {
            struct_id: NodeId,
            struct_created_at: DateTime<Utc>,
            struct_updated_at: DateTime<Utc>,
        }

        let struct_data = NodeBaseData::deserialize(deserializer)?;

        if struct_data.struct_updated_at < struct_data.struct_created_at {
            return Err(serde::de::Error::custom(
                "node updated_at cannot be earlier than created_at",
            ));
        }

        Ok(Self {
            struct_id: struct_data.struct_id,
            struct_created_at: struct_data.struct_created_at,
            struct_updated_at: struct_data.struct_updated_at,
        })
    }
}

/// Common structural data shared by all edges.
///
/// This base deliberately contains no information that imposes a directed,
/// undirected, or hypergraph structure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EdgeBase {
    struct_id: EdgeId,
    struct_created_at: DateTime<Utc>,
    struct_updated_at: DateTime<Utc>,
}

impl EdgeBase {
    /// Creates an edge base with a fresh identifier and current timestamps.
    pub fn new() -> Self {
        let struct_now = Utc::now();

        Self {
            struct_id: EdgeId::new(),
            struct_created_at: struct_now,
            struct_updated_at: struct_now,
        }
    }

    /// Returns the edge identifier.
    pub const fn struct_id(&self) -> &EdgeId {
        &self.struct_id
    }

    /// Returns the edge creation timestamp.
    pub const fn struct_created_at(&self) -> &DateTime<Utc> {
        &self.struct_created_at
    }

    /// Returns the edge update timestamp.
    pub const fn struct_updated_at(&self) -> &DateTime<Utc> {
        &self.struct_updated_at
    }

    /// Updates the edge modification timestamp without allowing it to regress.
    pub fn update_timestamp(&mut self) {
        let struct_now = Utc::now();

        if struct_now > self.struct_updated_at {
            self.struct_updated_at = struct_now;
        }
    }
}

impl Default for EdgeBase {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EdgeBase {
    fn deserialize<DeserializerType>(
        deserializer: DeserializerType,
    ) -> Result<Self, DeserializerType::Error>
    where
        DeserializerType: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EdgeBaseData {
            struct_id: EdgeId,
            struct_created_at: DateTime<Utc>,
            struct_updated_at: DateTime<Utc>,
        }

        let struct_data = EdgeBaseData::deserialize(deserializer)?;

        if struct_data.struct_updated_at < struct_data.struct_created_at {
            return Err(serde::de::Error::custom(
                "edge updated_at cannot be earlier than created_at",
            ));
        }

        Ok(Self {
            struct_id: struct_data.struct_id,
            struct_created_at: struct_data.struct_created_at,
            struct_updated_at: struct_data.struct_updated_at,
        })
    }
}
