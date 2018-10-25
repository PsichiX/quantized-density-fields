use id::ID;
use qdf::state::State;

/// Describes level data.
#[derive(Debug, Clone)]
pub enum LevelData {
    /// Level contains sublevels.
    SubLevels(Vec<ID>),
    /// Level contains QDF.
    Field(ID),
}

impl LevelData {
    /// Tells if level contains sublevels.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert!(lod.level(lod.root()).data().is_sublevels());
    /// ```
    #[inline]
    pub fn is_sublevels(&self) -> bool {
        match self {
            LevelData::SubLevels(_) => true,
            _ => false,
        }
    }

    /// Tells if level contains QDF.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// assert!(lod.level(lod.root()).data().is_field());
    /// ```
    #[inline]
    pub fn is_field(&self) -> bool {
        match self {
            LevelData::Field(_) => true,
            _ => false,
        }
    }

    /// Gets sublevels or panics if does not contains sublevels.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(lod.level(lod.root()).data().as_sublevels().len(), 4);
    /// ```
    #[inline]
    pub fn as_sublevels(&self) -> &[ID] {
        if let LevelData::SubLevels(sublevels) = self {
            sublevels
        } else {
            panic!("LevelData does not contains sublevels: {:?}", self);
        }
    }

    /// Gets QDF or panics if does not contains QDF.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// assert_eq!(*lod.field(lod.level(lod.root()).data().as_field()).state(), 16);
    /// ```
    #[inline]
    pub fn as_field(&self) -> ID {
        if let LevelData::Field(id) = self {
            *id
        } else {
            panic!("LevelData does not contains field: {:?}", self);
        }
    }
}

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
    data: LevelData,
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
            data: LevelData::SubLevels(vec![]),
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

    /// Gets level data.
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
