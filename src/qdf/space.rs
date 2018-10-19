use qdf::*;
use id::*;
use error::*;

#[derive(Debug, Clone)]
pub struct Space<S> where S: State {
    id: Id,
    parent: Option<Id>,
    state: S,
    subspace: Vec<Id>,
}

impl<S> Space<S> where S: State {
    #[inline]
    pub(crate) fn new(state: S) -> Self {
        Self {
            id: Id::new(),
            parent: None,
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub(crate) fn with_id(id: Id, state: S) -> Self {
        Self {
            id,
            parent: None,
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub(crate) fn with_id_parent_state(id: Id, parent: Id, state: S) -> Self {
        Space {
            id,
            parent: Some(parent),
            state,
            subspace: vec![],
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn parent(&self) -> Option<Id> {
        self.parent
    }

    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    #[inline]
    pub fn subspace<'a>(&'a self) -> &'a[Id] {
        &self.subspace
    }

    #[inline]
    pub fn is_platonic(&self) -> bool {
        self.subspace.is_empty()
    }

    #[inline]
    pub fn validate(&self, qdf: &QDF<S>) -> Result<()> {
        if !self.subspace.is_empty() && (self.subspace.len() != qdf.dimensions() + 1) {
            Err(QDFError::IncorrectDimensionsNumber(self.subspace.len()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub(crate) fn apply_subspace(&mut self, subspace: Vec<Id>) {
        self.subspace = subspace;
    }
}

impl<S> Default for Space<S> where S: State {
    #[inline]
    fn default() -> Self {
        Self::new(S::default())
    }
}
