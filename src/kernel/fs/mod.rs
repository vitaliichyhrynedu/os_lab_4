use bitmap::*;
use inode::*;

pub mod bitmap;
pub mod directory;
pub mod inode;

use crate::hardware::storage::{Storage, block::BLOCK_SIZE, block::Block};

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
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &fs.superblock as *const _ as *const u8,
                size_of::<Superblock>(),
            )
        };
        let block = unsafe { Block::try_from(bytes) }.expect("Superblock cannot exceed BLOCK_SIZE");
        storage
            .write_block(&block, 0)
            .map_err(|e| "Failed to write superblock: {e}")?;

        // Initialize metadata
        let mut inode_bitmap = Bitmap::new(inode_count);
        let mut block_bitmap = Bitmap::new(block_count);
        let mut inode_table = InodeTable::new(inode_count);

        // Allocate superblock
        block_bitmap.allocate(0)?;

        // Allocate and write inode bitmap
        let start = fs.superblock.inode_bitmap_start_block;
        let end = fs.superblock.block_bitmap_start_block;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                inode_bitmap.as_slice() as *const _ as *const u8,
                size_of::<Superblock>(),
            )
        };
        let blocks = unsafe { Block::from_bytes(bytes) };
        let indices = block_bitmap.allocate_span(start, end)?;
        storage
            .write_blocks(&blocks, &indices)
            .map_err(|_| "Failed to write inode bitmap")?;

        // Allocate and write the inode table
        let start = fs.superblock.inode_table_start_block;
        let end = fs.superblock.data_start_block;
        let indices = block_bitmap.allocate_span(start, end)?;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                inode_table.as_slice() as *const _ as *const u8,
                size_of::<Superblock>(),
            )
        };
        let blocks = unsafe { Block::from_bytes(bytes) };
        storage
            .write_blocks(&blocks, &indices)
            .map_err(|_| "Failed to write inode table")?;

        // Allocate and write the block bitmap
        let start = fs.superblock.block_bitmap_start_block;
        let end = fs.superblock.inode_table_start_block;
        let indices = block_bitmap.allocate_span(start, end)?;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                block_bitmap.as_slice() as *const _ as *const u8,
                size_of::<Superblock>(),
            )
        };
        let blocks = unsafe { Block::from_bytes(bytes) };
        storage
            .write_blocks(&blocks, &indices)
            .map_err(|_| "Failed to write block bitmap")?;

        Ok(fs)
    }
}

/// Represents metadata about the file system
#[repr(C)]
pub struct Superblock {
    pub block_count: u64,
    pub inode_count: u64,
    // Starting blocks for regions (in order)
    pub inode_bitmap_start_block: u64,
    pub block_bitmap_start_block: u64,
    pub inode_table_start_block: u64,
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
