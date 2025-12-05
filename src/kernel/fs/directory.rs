#[repr(C)]
pub struct DirectoryEntry {
    pub inumber: u64,
    pub filetype: FileType,
    pub name: Name,
}

impl DirectoryEntry {
    /// Constructs a directory entry with given parameters
    pub fn new(inumber: u64, filetype: FileType, name: Name) -> Self {
        Self {
            inumber,
            filetype,
            name,
        }
    }

    /// Constructs a `.` directory entry with a given inumber
    pub fn itself(inumber: u64) -> Self {
        Self::new(
            inumber,
            FileType::Directory,
            Name::new(".").expect("'.' must be a valid directory entry name"),
        )
    }

    /// Constructs a `..` directory entry with a given inumber
    pub fn parent(inumber: u64) -> Self {
        Self::new(
            inumber,
            FileType::Directory,
            Name::new("..").expect("'..' must be a valid directory entry name"),
        )
    }

    pub fn is_free(&self) -> bool {
        self.inumber == 0
    }
}

const MAX_NAME_LEN: usize = 64;

/// Represents the name of a directory entry
pub struct Name {
    bytes: [u8; MAX_NAME_LEN],
}

impl Name {
    /// Constructs a name from a string, if it doesn't exceed `MAX_NAME_LEN`
    pub fn new(string: &str) -> Option<Self> {
        let len = string.len();
        if len > MAX_NAME_LEN {
            return None;
        }
        let mut bytes = [0u8; MAX_NAME_LEN];
        bytes[..len].copy_from_slice(string.as_bytes());
        Some(Self { bytes })
    }
}

/// Represents file types
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FileType {
    File,
    Directory,
}
