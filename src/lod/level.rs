use id::ID;
use qdf::state::State;

#[derive(Debug, Clone)]
pub enum LevelData {
    SubLevels(Vec<ID>),
    Field(ID),
}

impl LevelData {
    #[inline]
    pub fn is_sublevels(&self) -> bool {
        match self {
            LevelData::SubLevels(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_field(&self) -> bool {
        match self {
            LevelData::Field(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn as_sublevels(&self) -> &[ID] {
        if let LevelData::SubLevels(sublevels) = self {
            sublevels
        } else {
            panic!("LevelData does not contains sublevels: {:?}", self);
        }
    }

    #[inline]
    pub fn as_field(&self) -> ID {
        if let LevelData::Field(id) = self {
            *id
        } else {
            panic!("LevelData does not contains field: {:?}", self);
        }
    }
}

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
    data: LevelData,
}

impl<S> Level<S> where S: State {
    #[inline]
    pub(crate) fn new(
        id: ID,
        parent: Option<ID>,
        level: usize,
        index: usize,
        state: S,
    ) -> Self {
        Self {
            id,
            parent,
            level,
            index,
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
    pub fn index(&self) -> usize {
        self.index
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
    pub(crate) fn apply_state(&mut self, state: S) {
        self.state = state;
    }

    #[inline]
    pub(crate) fn apply_data(&mut self, data: LevelData) {
        self.data = data;
    }
}
