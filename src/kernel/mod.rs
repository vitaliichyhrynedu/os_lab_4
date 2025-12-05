use crate::hardware::storage::Storage;

mod fs;

/// A model for the kernel
pub struct Kernel {
    storage: Storage,
}

impl Kernel {
    /// Constructs a kernel
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
