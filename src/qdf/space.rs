use id::*;
use qdf::*;

#[derive(Debug, Clone)]
pub struct Space<S>
where
    S: State,
{
    id: ID,
    parent: Option<ID>,
    state: S,
    subspace: Vec<ID>,
}

impl<S> Space<S>
where
    S: State,
{
    #[inline]
    pub(crate) fn new(state: S) -> Self {
        Self {
            id: ID::new(),
            parent: None,
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub(crate) fn with_id(id: ID, state: S) -> Self {
        Self {
            id,
            parent: None,
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub(crate) fn with_id_parent_state(id: ID, parent: ID, state: S) -> Self {
        Space {
            id,
            parent: Some(parent),
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn parent(&self) -> Option<ID> {
        self.parent
    }

    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    #[inline]
    pub fn subspace(&self) -> &[ID] {
        &self.subspace
    }

    #[inline]
    pub fn is_platonic(&self) -> bool {
        self.subspace.is_empty()
    }

    #[inline]
    pub(crate) fn apply_state(&mut self, state: S) {
        self.state = state;
    }

    #[inline]
    pub(crate) fn apply_subspace(&mut self, subspace: Vec<ID>) {
        self.subspace = subspace;
    }
}

impl<S> Default for Space<S>
where
    S: State,
{
    #[inline]
    fn default() -> Self {
        Self::new(S::default())
    }
}
