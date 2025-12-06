pub const BLOCK_SIZE: u64 = 4096;

/// A model for a physical storage device
pub struct Storage {
    blocks: Box<[Block]>,
}

impl Storage {
    /// Constructs a zero-initialized `Storage` of specified size and block size
    pub fn new(size: u64) -> Self {
        let block_count = size.div_ceil(BLOCK_SIZE);
        let blocks = vec![Block::new(); block_count as usize].into_boxed_slice();
        Self { blocks }
    }

    /// Returns the number of blocks
    pub fn block_count(&self) -> u64 {
        self.blocks.len() as u64
    }

    /// Reads data from the block at specified index into `dst` block
    pub fn read_block(&self, index: u64, dst: &mut Block) -> Result<(), Error> {
        let src = &self
            .blocks
            .get(index as usize)
            .ok_or(Error::BlockIndexOutOfBounds)?;
        *dst = **src;
        Ok(())
    }

    /// Writes data from the `src` block into the block at specified index
    pub fn write_block(&mut self, index: u64, src: &Block) -> Result<(), Error> {
        let dst = &mut self
            .blocks
            .get_mut(index as usize)
            .ok_or(Error::BlockIndexOutOfBounds)?;
        **dst = *src;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    BlockIndexOutOfBounds,
}

/// A model for a block
#[derive(Clone, Copy)]
pub struct Block {
    data: [u8; BLOCK_SIZE as usize],
}

impl Block {
    /// Constructs a zero-initialized `Block` of specified size
    pub fn new() -> Self {
        Self {
            data: [0u8; BLOCK_SIZE as usize],
        }
    }

    /// Creates a block from a pointer to a sized type if it fits
    pub unsafe fn from_sized<T: Sized>(data: &T) -> Result<Self, &'static str> {
        let size = size_of::<T>();
        if size > BLOCK_SIZE as usize {
            return Err("Sized type doesn't fit in a block");
        }
        let bytes = unsafe { std::slice::from_raw_parts(data as *const _ as *const u8, size) };
        Ok(Block::from(bytes))
    }

    /// Returns a pointer to the data inside the block, interpreting it as a sized type `T`
    ///
    /// # Safety: T must be #[repr(C)]
    pub unsafe fn as_sized<T: Sized>(&self) -> Result<&T, &'static str> {
        let size = size_of::<T>();
        if size > BLOCK_SIZE as usize {
            return Err("Sized type doesn't fit in a block");
        }
        Ok(unsafe { &*(self.data.as_ptr() as *const T) })
    }
}

impl From<&[u8]> for Block {
    /// Creates a block from a slice of bytes
    ///
    /// Panics if the slice is too large, pads with zeros if too small
    fn from(bytes: &[u8]) -> Self {
        let mut block = Self::new();
        let length = bytes.len().min(BLOCK_SIZE as usize);
        block.data[0..length].copy_from_slice(&bytes[..length]);
        block
    }
}
