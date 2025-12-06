const MAX_EXTENTS: usize = 8;

/// Represents the table of all inodes
pub struct InodeTable {
    inodes: Vec<Inode>,
}

impl InodeTable {
    /// Constructs an empty inode table
    pub fn new(count: u64) -> Self {
        Self {
            inodes: vec![Inode::new(); count as usize],
        }
    }

    /// Get mutable reference to the inode at a given index
    pub fn get_mut(&mut self, index: u64) -> &mut Inode {
        self.inodes
            .get_mut(index as usize)
            .expect("Index out of bounds")
    }

    /// Returns the inode table as a slice of inodes
    pub fn as_slice(&self) -> &[Inode] {
        &self.inodes
    }

    /// Constructs an InodeTable from a slice of bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let inode_size = size_of::<Inode>();
        let count = bytes.len() / inode_size;
        let mut inodes = Vec::with_capacity(count);
        for chunk in bytes.chunks_exact(inode_size) {
            let inode = unsafe { std::ptr::read_unaligned(chunk.as_ptr() as *const Inode) };
            inodes.push(inode);
        }
        InodeTable { inodes }
    }
}

/// Represents a file system object
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Inode {
    pub size: u64,
    pub link_count: u8,
    pub extents: [Extent; MAX_EXTENTS],
}

impl Inode {
    pub fn new() -> Self {
        Self {
            size: 0,
            link_count: 0,
            extents: [Extent::default(); MAX_EXTENTS],
        }
    }
}

/// Represents a contigous range of blocks
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Extent {
    pub start_block: u64,
    pub length: u64,
}
