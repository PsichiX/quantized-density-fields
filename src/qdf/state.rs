use std::fmt::Debug;

/// Trait that describes QDF space state.
pub trait State: Sized + Clone + Default + Debug + Subdividable {}

impl State for () {}
impl State for bool {}
impl State for i8 {}
impl State for i16 {}
impl State for i32 {}
impl State for i64 {}
impl State for u8 {}
impl State for u16 {}
impl State for u32 {}
impl State for u64 {}
impl State for f32 {}
impl State for f64 {}
impl State for isize {}
impl State for usize {}

/// Trait that describes subdividable data.
///
/// # Examples
/// ```
/// use quantized_density_fields::Subdividable;
/// use std::iter::repeat;
///
/// #[derive(Debug, Eq, PartialEq, Clone)]
/// struct Integer(i32);
///
/// impl Subdividable for Integer {
///     fn subdivide(&self, subdivisions: usize) -> Self {
///         Integer(self.0 / subdivisions as i32)
///     }
///     fn merge(states: &[Self]) -> Self {
///         Integer(states.iter().map(|v| v.0).sum())
///     }
/// }
///
/// let substate = Integer(16).subdivide(4);
/// assert_eq!(substate, Integer(4));
/// let state = Subdividable::merge(&repeat(substate).take(4).collect::<Vec<Integer>>());
/// assert_eq!(state, Integer(16));
/// ```
pub trait Subdividable: Sized {
    /// Create data template that we get by subdivision of source data.
    ///
    /// # Arguments
    /// * `subdivisions` - number of subdivisions.
    fn subdivide(&self, subdivisions: usize) -> Self;
    /// Merge multiple data instances into one.
    ///
    /// # Arguments
    /// * `states` - list of source data to merge.
    fn merge(states: &[Self]) -> Self;
}

impl Subdividable for () {
    fn subdivide(&self, _: usize) -> Self {
        ()
    }
    fn merge(_: &[Self]) -> Self {
        ()
    }
}
impl Subdividable for bool {
    fn subdivide(&self, _: usize) -> Self {
        *self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().any(|v| *v)
    }
}
impl Subdividable for i8 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for i16 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for i32 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for i64 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for u8 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for u16 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for u32 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for u64 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for f32 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for f64 {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for isize {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
impl Subdividable for usize {
    fn subdivide(&self, subdivisions: usize) -> Self {
        self / subdivisions as Self
    }
    fn merge(states: &[Self]) -> Self {
        states.iter().sum()
    }
}
