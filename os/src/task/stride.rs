use core::cmp::Ordering;

/// potential change

#[derive(Ord, Eq, PartialEq)]
pub struct Stride {
    pub stride: usize,
    pub pid: usize,
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.stride.partial_cmp(&self.stride)
    }
}