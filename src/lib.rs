pub mod client;
pub mod types;

pub use types::{BlockId, BlockTag};

pub use client::StupidEthClient;

// convenience enums
pub const PENDING: BlockId = BlockId::Tag(BlockTag::PENDING);
pub const EARLIEST: BlockId = BlockId::Tag(BlockTag::EARLIEST);
pub const LATEST: BlockId = BlockId::Tag(BlockTag::LATEST);
pub const SAFE: BlockId = BlockId::Tag(BlockTag::SAFE);
pub const FINALIZED: BlockId = BlockId::Tag(BlockTag::FINALIZED);
