use alloy_primitives::U256;
use serde::{Deserialize, Serialize, Serializer};

// Define an enum for the block parameter with custom serialization
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockId {
    Number(U256),
    Tag(BlockTag),
}
#[derive(Debug, Clone, Deserialize)]
pub enum BlockTag {
    PENDING,
    LATEST,
    SAFE,
    FINALIZED,
    EARLIEST,
}
// Implement custom serialization for BlockTag to serialize it as lowercase string
impl Serialize for BlockTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self).to_lowercase())
    }
}

impl From<U256> for BlockId {
    fn from(number: U256) -> Self {
        BlockId::Number(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_block_tag() {
        let tag = BlockTag::LATEST;
        let serialized = serde_json::to_string(&tag).unwrap();
        assert_eq!(serialized, "\"latest\"");
    }

    #[test]
    fn serialize_block_id_number() {
        let block_number = U256::from(0);
        let number = BlockId::from(block_number);
        let serialized = serde_json::to_string(&number).unwrap();
        assert_eq!(serialized, "\"0x0\"");
    }

    #[test]
    fn serialize_block_id_tag() {
        let tag = BlockId::Tag(BlockTag::EARLIEST);
        let serialized = serde_json::to_string(&tag).unwrap();
        assert_eq!(serialized, "\"earliest\"");
    }
}
