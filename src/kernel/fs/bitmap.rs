/// Tracks allocation state of some objects
pub struct Bitmap {
    allocs: Vec<Allocation>,
}

impl Bitmap {
    /// Constructs a zero-initialized bitmap
    pub fn new(obj_count: u64) -> Self {
        Bitmap {
            allocs: vec![Allocation::default(); obj_count as usize],
        }
    }

    /// Finds the index of the first free object
    pub fn find_free(&mut self) -> Option<u64> {
        for (index, allocation) in self.allocs.iter().enumerate() {
            if let Allocation::Free = allocation {
                return Some(index as u64);
            }
        }
        None
    }

    /// Marks the object at an index as used
    pub fn allocate(&mut self, index: u64) -> Result<(), &'static str> {
        let obj = self
            .allocs
            .get_mut(index as usize)
            .ok_or("Index out of bounds")?;
        *obj = Allocation::Used;
        Ok(())
    }

    /// Marks objects at the start..end indices as used
    pub fn allocate_span(&mut self, start: u64, end: u64) -> Result<(), &'static str> {
        if end as usize > self.allocs.len() {
            return Err("Span out of bounds");
        };
        for i in start..end {
            self.allocate(i)?;
        }
        Ok(())
    }

    /// Marks the object at index as free
    pub fn free(&mut self, index: u64) -> Result<(), &'static str> {
        let obj = self
            .allocs
            .get_mut(index as usize)
            .ok_or("Index out of bounds")?;
        *obj = Allocation::Free;
        Ok(())
    }

    /// Returns the bitmap as a slice of allocations
    pub fn as_slice(&self) -> &[Allocation] {
        &self.allocs
    }
}

/// Represents allocation state of an object
#[derive(Default, Clone, Copy)]
#[repr(u8)]
pub enum Allocation {
    #[default]
    Free,
    Used,
}
