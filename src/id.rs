use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Erreur retournée lorsqu'un identifiant de graphe est invalide.
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

/// Identifiant unique et fortement typé d'un nœud.
///
/// La représentation interne est entièrement encapsulée par `common_graph`.
/// Les nouveaux identifiants sont toujours générés en UUIDv7.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    fn parse(string_value: &str) -> Result<Self, GraphIdParseError> {
        let struct_uuid = Uuid::parse_str(string_value).map_err(|struct_error| {
            GraphIdParseError::new(format!("identifiant de nœud invalide : {struct_error}"))
        })?;

        if struct_uuid.get_version_num() != 7 {
            return Err(GraphIdParseError::new(
                "un identifiant de nœud doit être un UUIDv7",
            ));
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

/// Identifiant unique et fortement typé d'une arête.
///
/// `EdgeId` reste distinct de `NodeId`, ce qui empêche Rust de les mélanger.
/// La représentation interne est entièrement encapsulée par `common_graph`.
/// Les nouveaux identifiants sont toujours générés en UUIDv7.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EdgeId(Uuid);

impl EdgeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    fn parse(string_value: &str) -> Result<Self, GraphIdParseError> {
        let struct_uuid = Uuid::parse_str(string_value).map_err(|struct_error| {
            GraphIdParseError::new(format!("identifiant d'arête invalide : {struct_error}"))
        })?;

        if struct_uuid.get_version_num() != 7 {
            return Err(GraphIdParseError::new(
                "un identifiant d'arête doit être un UUIDv7",
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
