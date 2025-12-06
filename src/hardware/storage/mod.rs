use super::block::{BLOCK_SIZE, Block};

pub mod block;

/// A model of a physical storage device
pub struct Storage {
    blocks: Box<[Block]>,
}

impl Storage {
    /// Constructs a zero-initialized `Storage` of given size specified in bytes
    pub fn new(size: u64) -> Self {
        let block_count = size.div_ceil(BLOCK_SIZE);
        let blocks = vec![Block::new(); block_count as usize].into_boxed_slice();
        Self { blocks }
    }

    /// Returns the number of blocks
    pub fn block_count(&self) -> u64 {
        self.blocks.len() as u64
    }

    /// Reads data from the persistent block at specified index into `dst` block
    /// Returns an error if the index is out of bounds.
    pub fn read_block(&self, index: u64, dst: &mut Block) -> Result<(), Error> {
        let src = &self
            .blocks
            .get(index as usize)
            .ok_or(Error::BlockIndexOutOfBounds)?;
        *dst = **src;
        Ok(())
    }

    /// Writes data from the `src` block into persistent block at specified index.
    /// Returns an error if the index is out of bounds.
    pub fn write_block(&mut self, src: &Block, index: u64) -> Result<(), Error> {
        let dst = &mut self
            .blocks
            .get_mut(index as usize)
            .ok_or(Error::BlockIndexOutOfBounds)?;
        **dst = *src;
        Ok(())
    }

    /// Writes data from the 'srcs' blocks into persistent blocks at specified indeces
    /// Returns an error if the index is out of bounds.
    pub fn write_blocks(&mut self, srcs: &[Block], indeces: &[u64]) -> Result<(), Error> {
        for (src, &index) in srcs.iter().zip(indeces.iter()) {
            self.write_block(src, index)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    BlockIndexOutOfBounds,
}
