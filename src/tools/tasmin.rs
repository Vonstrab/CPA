use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub density: usize,
    pub node: u32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .density
            .cmp(&self.density)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    #[inline]

    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
