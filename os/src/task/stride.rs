use core::cmp::Ordering;

/// potential change
#[derive(Ord, Eq)]
pub struct Stride {
    pub stride: usize,
    pub pid: usize,
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.stride == other.stride
        { other.pid.partial_cmp(&self.pid) } else { other.stride.partial_cmp(&self.stride) }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}