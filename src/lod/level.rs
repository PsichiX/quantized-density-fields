use id::ID;
use qdf::state::State;

/// Holds information about space level.
#[derive(Debug, Clone)]
pub struct Level<S>
where
    S: State,
{
    id: ID,
    parent: Option<ID>,
    level: usize,
    index: usize,
    state: S,
    sublevels: Vec<ID>,
}

impl<S> Level<S>
where
    S: State,
{
    #[inline]
    pub(crate) fn new(id: ID, parent: Option<ID>, level: usize, index: usize, state: S) -> Self {
        Self {
            id,
            parent,
            level,
            index,
            state,
            sublevels: vec![],
        }
    }

    /// Gets level id.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets level parent id or `None` if it is root level.
    #[inline]
    pub fn parent(&self) -> Option<ID> {
        self.parent
    }

    /// Gets zoom level index.
    #[inline]
    pub fn level(&self) -> usize {
        self.level
    }

    /// Tells level index in parent.
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Gets level state.
    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Gets level sublevels.
    #[inline]
    pub fn sublevels(&self) -> &[ID] {
        &self.sublevels
    }

    #[inline]
    pub fn apply_state(&mut self, state: S) {
        self.state = state;
    }

    #[inline]
    pub(crate) fn apply_sublevels(&mut self, sublevels: Vec<ID>) {
        self.sublevels = sublevels;
    }
}
