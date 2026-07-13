use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Error returned when a graph identifier is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphIdParseError {
    string_message: String,
}

impl GraphIdParseError {
    fn new(string_message: impl Into<String>) -> Self {
        Self {
            string_message: string_message.into(),
        }
    }
}

impl fmt::Display for GraphIdParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.string_message)
    }
}

impl std::error::Error for GraphIdParseError {}

/// Unique and strongly typed node identifier.
///
/// The internal representation is fully encapsulated by `common_graph`.
/// New identifiers are always generated as `UUIDv7`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Generates a new node identifier as `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    fn parse(string_value: &str) -> Result<Self, GraphIdParseError> {
        let struct_uuid = Uuid::parse_str(string_value).map_err(|struct_error| {
            GraphIdParseError::new(format!("invalid node identifier: {struct_error}"))
        })?;

        if struct_uuid.get_version_num() != 7 {
            return Err(GraphIdParseError::new("a node identifier must be a UUIDv7"));
        }

        Ok(Self(struct_uuid))
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for NodeId {
    type Err = GraphIdParseError;

    fn from_str(string_value: &str) -> Result<Self, Self::Err> {
        Self::parse(string_value)
    }
}

impl Serialize for NodeId {
    fn serialize<SerializerType>(
        &self,
        serializer: SerializerType,
    ) -> Result<SerializerType::Ok, SerializerType::Error>
    where
        SerializerType: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<DeserializerType>(
        deserializer: DeserializerType,
    ) -> Result<Self, DeserializerType::Error>
    where
        DeserializerType: Deserializer<'de>,
    {
        let string_value = String::deserialize(deserializer)?;

        Self::parse(&string_value).map_err(serde::de::Error::custom)
    }
}

/// Unique and strongly typed edge identifier.
///
/// `EdgeId` remains distinct from `NodeId`, preventing accidental mixing.
/// The internal representation is fully encapsulated by `common_graph`.
/// New identifiers are always generated as `UUIDv7`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EdgeId(Uuid);

impl EdgeId {
    /// Generates a new edge identifier as `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    fn parse(string_value: &str) -> Result<Self, GraphIdParseError> {
        let struct_uuid = Uuid::parse_str(string_value).map_err(|struct_error| {
            GraphIdParseError::new(format!("invalid edge identifier: {struct_error}"))
        })?;

        if struct_uuid.get_version_num() != 7 {
            return Err(GraphIdParseError::new(
                "an edge identifier must be a UUIDv7",
            ));
        }

        Ok(Self(struct_uuid))
    }
}

impl Default for EdgeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for EdgeId {
    type Err = GraphIdParseError;

    fn from_str(string_value: &str) -> Result<Self, Self::Err> {
        Self::parse(string_value)
    }
}

impl Serialize for EdgeId {
    fn serialize<SerializerType>(
        &self,
        serializer: SerializerType,
    ) -> Result<SerializerType::Ok, SerializerType::Error>
    where
        SerializerType: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EdgeId {
    fn deserialize<DeserializerType>(
        deserializer: DeserializerType,
    ) -> Result<Self, DeserializerType::Error>
    where
        DeserializerType: Deserializer<'de>,
    {
        let string_value = String::deserialize(deserializer)?;

        Self::parse(&string_value).map_err(serde::de::Error::custom)
    }
}
