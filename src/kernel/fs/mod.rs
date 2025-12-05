pub mod inode;

use std::slice;

use inode::*;

pub mod bitmap;
use bitmap::*;

pub mod directory;

use crate::hardware::storage::{BLOCK_SIZE, Block, Storage};

pub const ROOT_INUMBER: u64 = 1;

/// A model for the file system
pub struct FileSystem {
    pub superblock: Superblock,
}

impl FileSystem {
    /// Formats the storage device, creating a file system
    pub fn format(
        storage: &mut Storage,
        block_count: u64,
        inode_count: u64,
    ) -> Result<Self, &'static str> {
        let fs = Self {
            superblock: Superblock::new(BLOCK_SIZE, block_count, inode_count),
        };
        let block = unsafe { Block::from_sized(&fs.superblock) }?;
        storage
            .write_block(0, &block)
            .map_err(|_| "Failed to write superblock")?;

        let mut block_bitmap = Bitmap::new(block_count);
        let inode_bitmap = Bitmap::new(inode_count);
        let inode_table = InodeTable::new(inode_count);

        // Mark the blocks belonging to the metadata regions as used
        block_bitmap.allocate_span(0, fs.superblock.data_start_block)?;

        // TODO: Create a root directory

        // Write metadata to storage
        Self::write_slice(
            storage,
            fs.superblock.block_bitmap_start_block,
            block_bitmap.as_slice(),
        )
        .map_err(|_| "Failed to write block bitmap")?;
        Self::write_slice(
            storage,
            fs.superblock.inode_bitmap_start_block,
            inode_bitmap.as_slice(),
        )
        .map_err(|_| "Failed to write inode bitmap")?;
        Self::write_slice(
            storage,
            fs.superblock.inode_table_start_block,
            inode_table.as_slice(),
        )
        .map_err(|_| "Failed to write inode table")?;

        Ok(fs)
    }

    /// Writes a slice to storage, fitting its contents into blocks
    fn write_slice<T>(storage: &mut Storage, start_block: u64, data: &[T]) -> Result<(), String> {
        let size_of_val = std::mem::size_of_val(data);
        unsafe {
            let data = data.as_ptr() as *const u8;
            let bytes = slice::from_raw_parts(data, size_of_val);
            for (i, chunk) in bytes.chunks(BLOCK_SIZE as usize).enumerate() {
                let block = Block::from(chunk);
                storage
                    .write_block(start_block + i as u64, &block)
                    .map_err(|_| "Failed to write slice")?
            }
        }
        Ok(())
    }
}

/// Represents metadata about the file system
#[repr(C)]
pub struct Superblock {
    pub block_count: u64,
    pub inode_count: u64,
    // Starting blocks for metadata regions
    pub block_bitmap_start_block: u64,
    pub inode_bitmap_start_block: u64,
    pub inode_table_start_block: u64,
    // Starting block for the data region
    pub data_start_block: u64,
}

impl Superblock {
    /// Constructs a superblock with specified block size, block and inode count
    pub fn new(block_size: u64, block_count: u64, inode_count: u64) -> Self {
        // Superblock lives in the first block
        let inode_bitmap_start_block = 1;
        let inode_bitmap_bytes = inode_count * (size_of::<Allocation>() as u64);
        let inode_bitmap_blocks = inode_bitmap_bytes.div_ceil(block_size);

        let block_bitmap_bytes = block_count * (size_of::<Allocation>() as u64);
        let block_bitmap_blocks = block_bitmap_bytes.div_ceil(block_size);
        let block_bitmap_start_block = inode_bitmap_start_block + inode_bitmap_blocks;

        let inode_table_bytes = inode_count * (size_of::<Inode>() as u64);
        let inode_table_blocks = inode_table_bytes.div_ceil(block_size);
        let inode_table_start_block = block_bitmap_start_block + block_bitmap_blocks;

        let data_start_block = inode_table_start_block + inode_table_blocks;

        Self {
            block_count,
            inode_count,
            inode_bitmap_start_block,
            block_bitmap_start_block,
            inode_table_start_block,
            data_start_block,
        }
    }
}
