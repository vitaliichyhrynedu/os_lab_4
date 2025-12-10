use zerocopy::{FromBytes, Immutable, IntoBytes};

/// Block size specified in bytes.
pub const BLOCK_SIZE: usize = 4096;

/// A fixed-sized byte sequence.
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(FromBytes, IntoBytes, Immutable)]
pub struct Block {
    pub data: [u8; BLOCK_SIZE],
}

impl Block {
    /// Constructs a zero-initialized [Block].
    pub fn new() -> Self {
        Self {
            data: [0u8; BLOCK_SIZE],
        }
    }

    /// Casts a byte slice into a [Block] slice without copying.
    ///
    /// # Panics
    /// Panics if `bytes.len()` is not a multiple of [BLOCK_SIZE].
    pub fn slice_from_bytes(bytes: &[u8]) -> &[Self] {
        <[Self]>::ref_from_bytes(bytes).unwrap()
    }
}
