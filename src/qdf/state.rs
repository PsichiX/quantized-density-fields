use std::fmt::Debug;

/// Trait that describes QDF space state.
///
/// # Examples
/// ```
/// use quantized_density_fields::State;
/// use std::iter::repeat;
///
/// #[derive(Debug, Default, Eq, PartialEq, Clone)]
/// struct Integer(i32);
///
/// impl State for Integer {
///     fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
///         repeat(Integer(self.0 / subdivisions as i32)).take(subdivisions).collect()
///     }
///     fn merge(states: &[Self]) -> Self {
///         Integer(states.iter().map(|v| v.0).sum())
///     }
/// }
///
/// let substates = Integer(16).subdivide(4);
/// assert_eq!(substates, vec![Integer(4), Integer(4), Integer(4), Integer(4)]);
/// let state = State::merge(&substates);
/// assert_eq!(state, Integer(16));
/// ```
pub trait State: Sized + Clone + Default + Send + Sync + Debug {
    /// Create data template that we get by subdivision of source data.
    ///
    /// # Arguments
    /// * `subdivisions` - number of subdivisions.
    fn subdivide(&self, subdivisions: usize) -> Vec<Self>;
    /// Merge multiple data instances into one.
    ///
    /// # Arguments
    /// * `states` - list of source data to merge.
    fn merge(states: &[Self]) -> Self;
    /// Multiply and merge multiple instances of itself into one super state.
    ///
    /// # Arguments
    /// * `dimensions` - number of dimensions.
    /// * `level` - number level at which you merge.
    fn super_state_at_level(&self, dimensions: usize, level: usize) -> Self {
        let states = std::iter::repeat(self.clone())
            .take((dimensions + 1)
            .pow(level as u32))
            .collect::<Vec<Self>>();
        Self::merge(&states)
    }
}

impl State for i8 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for i16 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for i32 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for i64 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for u8 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for u16 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for u32 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for u64 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for f32 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for f64 {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for isize {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl State for usize {
    fn subdivide(&self, subdivisions: usize) -> Vec<Self> {
        ::std::iter::repeat(self / subdivisions as Self)
            .take(subdivisions)
            .collect()
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
