use id::*;
use qdf::*;

/// Holds information about space region.
#[derive(Debug, Clone)]
pub struct Space<S>
where
    S: State,
{
    id: ID,
    state: S,
}

impl<S> Space<S>
where
    S: State,
{
    #[inline]
    pub(crate) fn new(id: ID, state: S) -> Self {
        Self { id, state }
    }

    /// Gets space id.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets space state.
    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    #[inline]
    pub(crate) fn apply_state(&mut self, state: S) {
        self.state = state;
    }
}

impl<S> Default for Space<S>
where
    S: State,
{
    #[inline]
    fn default() -> Self {
        Self::new(ID::new(), S::default())
    }
}
