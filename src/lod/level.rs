use id::ID;
use qdf::state::State;

#[derive(Debug, Clone)]
pub enum LevelData {
    SubLevels(Vec<ID>),
    Field(ID),
}

#[derive(Debug, Clone)]
pub struct Level<S>
where
    S: State,
{
    id: ID,
    parent: Option<ID>,
    level: usize,
    state: S,
    data: LevelData,
}

impl<S> Level<S> where S: State {
    #[inline]
    pub(crate) fn new(
        id: ID,
        parent: Option<ID>,
        level: usize,
        state: S,
    ) -> Self {
        Self {
            id,
            parent,
            level,
            state,
            data: LevelData::SubLevels(vec![]),
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
    pub fn level(&self) -> usize {
        self.level
    }

    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    #[inline]
    pub fn data(&self) -> &LevelData {
        &self.data
    }

    #[inline]
    pub(crate) fn apply_data(&mut self, data: LevelData) {
        self.data = data;
    }
}
