pub const BLOCK_SIZE: u64 = 4096;

/// A model of a block
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Block {
    data: [u8; BLOCK_SIZE as usize],
}

impl Block {
    /// Constructs a zero-initialized `Block`
    pub fn new() -> Self {
        Self {
            data: [0u8; BLOCK_SIZE as usize],
        }
    }

    /// Interpretes bytes from a slice of blocks as a single slice of bytes
    pub fn as_bytes(blocks: &[Block]) -> &[u8] {
        let size = blocks.len() * BLOCK_SIZE as usize;
        unsafe { std::slice::from_raw_parts(blocks.as_ptr() as *const u8, size) }
    }

    /// Creates a vector of blocks from a slice of bytes
    pub fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        let block_count = bytes.len().div_ceil(BLOCK_SIZE as usize);
        let mut blocks = Vec::with_capacity(block_count);
        for chunk in bytes.chunks(BLOCK_SIZE as usize) {
            blocks.push(Block::try_from(chunk).expect("Chunk cannot be larger than BLOCK_SIZE"));
        }
        blocks
    }
}

impl TryFrom<&[u8]> for Block {
    type Error = BlockError;

    /// Tries to create a block from a slice of bytes.
    /// Returns an error if bytes do not fit in a block.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let length = bytes.len();
        if length > BLOCK_SIZE as usize {
            return Err(BlockError::BlockSizeExceeded);
        }
        let mut block = Self::new();
        block.data[..length].copy_from_slice(&bytes);
        Ok(block)
    }
}

#[derive(Debug)]
pub enum BlockError {
    BlockSizeExceeded,
}
