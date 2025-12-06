/// Tracks allocation state of some objects
pub struct Bitmap {
    allocs: Vec<Allocation>,
}

impl Bitmap {
    /// Constructs a zero-initialized bitmap that consists of a given object count
    pub fn new(count: u64) -> Self {
        Bitmap {
            allocs: vec![Allocation::default(); count as usize],
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

    /// Marks the object at an index as used and returns the index
    pub fn allocate(&mut self, index: u64) -> Result<u64, &'static str> {
        let obj = self
            .allocs
            .get_mut(index as usize)
            .ok_or("Index out of bounds")?;
        *obj = Allocation::Used;
        Ok(index)
    }

    /// Marks objects at the start..end indices as used and returns the indices
    pub fn allocate_span(&mut self, start: u64, end: u64) -> Result<Vec<u64>, &'static str> {
        if start > end {
            return Err("Invalid span");
        } else if end as usize > self.allocs.len() {
            return Err("Span out of bounds");
        };
        let mut indices = Vec::with_capacity((end - start) as usize);
        for i in start..end {
            indices.push(self.allocate(i)?);
        }
        Ok(indices)
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
