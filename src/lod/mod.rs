pub mod level;
mod tests;

pub use self::level::*;
use error::*;
use id::*;
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::astar;
use std::collections::HashMap;
use qdf::*;

/// Object that represents space level of details.
/// This gives you the ability to sample space area states at different zoom levels (LOD mechanism).
#[derive(Debug)]
pub struct LOD<S> where S: State {
    id: ID,
    graph: UnGraphMap<ID, ()>,
    levels: HashMap<ID, Level<S>>,
    fields: HashMap<ID, QDF<S>>,
    root: ID,
    dimensions: usize,
    count: usize,
}

impl<S> LOD<S>
where
    S: State,
{
    /// Creates new LOD information universe.
    ///
    /// # Arguments
    /// * `dimensions` - Number of dimensions which space contains.
    /// * `count` - Number of levels.
    /// * `root_state` - State of root level.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// // Create 2D space with 1 level of details and `16` as root space.
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(*lod.state(), 16);
    /// // LOD has 4 children level objects.
    /// assert_eq!(lod.level(lod.root()).data().as_sublevels().len(), 4);
    /// // sampled state at level 1 equals to `4` (`16 / 4`).
    /// assert_eq!(*lod.level(lod.level(lod.root()).data().as_sublevels()[0]).state(), 4);
    /// ```
    pub fn new(dimensions: usize, count: usize, root_state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut levels = HashMap::new();
        let mut fields = HashMap::new();
        let root = ID::new();
        let main = Level::new(root, None, 0, 0, root_state);
        levels.insert(root, main);
        graph.add_node(root);
        Self::subdivide_level(root, &mut graph, &mut levels, &mut fields, dimensions + 2, count);
        Self::connect_clusters(root, &mut graph, &levels);
        Self {
            id: ID::new(),
            graph,
            levels,
            fields,
            root,
            dimensions,
            count,
        }
    }

    /// Gets LOD id.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets LOD root level node id.
    #[inline]
    pub fn root(&self) -> ID {
        self.root
    }

    /// Gets LOD dimensions number.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(lod.dimensions(), 2);
    /// ```
    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    /// Gets LOD zoom levels number.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(lod.levels_count(), 1);
    /// ```
    #[inline]
    pub fn levels_count(&self) -> usize {
        self.count
    }

    /// Gets LOD root level state.
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(*lod.state(), 16);
    /// ```
    #[inline]
    pub fn state(&self) -> &S {
        self.levels[&self.root].state()
    }

    /// Tells if space level with given id exists in LOD.
    ///
    /// # Arguments
    /// * `id` - level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert!(lod.level_exists(lod.root()));
    /// ```
    #[inline]
    pub fn level_exists(&self, id: ID) -> bool {
        self.levels.contains_key(&id)
    }

    /// Try to get reference to given space level.
    ///
    /// # Arguments
    /// * `id` - level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// if let Some(level) = lod.try_get_level(lod.root()) {
    ///     assert_eq!(*level.state(), 16);
    /// }
    /// ```
    #[inline]
    pub fn try_get_level(&self, id: ID) -> Option<&Level<S>> {
        self.levels.get(&id)
    }

    /// Gets reference to given space level and throws error if level does not exists.
    ///
    /// # Arguments
    /// * `id` - level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// if let Ok(level) = lod.get_level(lod.root()) {
    ///     assert_eq!(*level.state(), 16);
    /// }
    /// ```
    #[inline]
    pub fn get_level(&self, id: ID) -> Result<&Level<S>> {
        if let Some(level) = self.levels.get(&id) {
            Ok(level)
        } else {
            Err(QDFError::LevelDoesNotExists(id))
        }
    }

    /// Gets reference to given space level and panics if level does not exists.
    ///
    /// # Arguments
    /// * `id` - level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// assert_eq!(*lod.level(lod.root()).state(), 16);
    /// ```
    #[inline]
    pub fn level(&self, id: ID) -> &Level<S> {
        &self.levels[&id]
    }

    /// Tells if QDF with given id exists in LOD.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// assert!(lod.field_exists(lod.level(lod.root()).data().as_field()));
    /// ```
    #[inline]
    pub fn field_exists(&self, id: ID) -> bool {
        self.fields.contains_key(&id)
    }

    /// Try to get QDF with given id.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// if let Some(qdf) = lod.try_get_field(lod.level(lod.root()).data().as_field()) {
    ///     assert_eq!(*qdf.state(), 16);
    /// }
    /// ```
    #[inline]
    pub fn try_get_field(&self, id: ID) -> Option<&QDF<S>> {
        self.fields.get(&id)
    }

    /// Try to get mutable QDF with given id.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 0, 16);
    /// let id = lod.level(lod.root()).data().as_field();
    /// if let Some(qdf) = lod.try_get_field_mut(id) {
    ///     qdf.set_space_state(id, 4);
    /// }
    /// ```
    #[inline]
    pub fn try_get_field_mut(&mut self, id: ID) -> Option<&mut QDF<S>> {
        self.fields.get_mut(&id)
    }

    /// Gets QDF with given id and throws error if field does not exists.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// if let Ok(qdf) = lod.get_field(lod.level(lod.root()).data().as_field()) {
    ///     assert_eq!(*qdf.state(), 16);
    /// }
    /// ```
    #[inline]
    pub fn get_field(&self, id: ID) -> Result<&QDF<S>> {
        if let Some(field) = self.fields.get(&id) {
            Ok(field)
        } else {
            Err(QDFError::FieldDoesNotExists(id))
        }
    }

    /// Gets mutable QDF with given id and throws error if field does not exists.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 0, 16);
    /// let id = lod.level(lod.root()).data().as_field();
    /// if let Ok(qdf) = lod.get_field_mut(id) {
    ///     qdf.set_space_state(id, 4);
    /// }
    /// ```
    #[inline]
    pub fn get_field_mut(&mut self, id: ID) -> Result<&mut QDF<S>> {
        if let Some(field) = self.fields.get_mut(&id) {
            Ok(field)
        } else {
            Err(QDFError::FieldDoesNotExists(id))
        }
    }

    /// Gets QDF with given id and panics if field does not exists.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 0, 16);
    /// assert_eq!(*lod.field(lod.level(lod.root()).data().as_field()).state(), 16);
    /// ```
    #[inline]
    pub fn field(&self, id: ID) -> &QDF<S> {
        &self.fields[&id]
    }

    /// Gets mutable QDF with given id and panics if field does not exists.
    ///
    /// # Arguments
    /// * `id` - QDF id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 0, 16);
    /// let id = lod.level(lod.root()).data().as_field();
    /// let mut qdf = lod.field_mut(id);
    /// let id = qdf.root();
    /// qdf.set_space_state(id, 4);
    /// ```
    #[inline]
    pub fn field_mut(&mut self, id: ID) -> &mut QDF<S> {
        self.fields.get_mut(&id).unwrap()
    }

    /// Gets list of space level neighbors IDs or throws error if level does not exists.
    ///
    /// # Arguments
    /// * `id` - Level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// let subs = lod.level(lod.root()).data().as_sublevels();
    /// assert_eq!(lod.find_level_neighbors(subs[0]).unwrap(), vec![subs[1], subs[2], subs[3]]);
    /// ```
    #[inline]
    pub fn find_level_neighbors(&self, id: ID) -> Result<Vec<ID>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::LevelDoesNotExists(id))
        }
    }

    /// Gets list of space level IDs that defines shortest path between two space levels,
    /// or throws error if level does not exists. Levels must lay on the same zoom level!
    ///
    /// # Arguments
    /// * `from` - source level id.
    /// * `to` - target level id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let lod = LOD::new(2, 1, 16);
    /// let subs = lod.level(lod.root()).data().as_sublevels();
    /// assert_eq!(lod.find_path(subs[1], subs[3]).unwrap(), vec![subs[1], subs[0], subs[3]]);
    /// ```
    pub fn find_path(&self, from: ID, to: ID) -> Result<Vec<ID>> {
        if !self.level_exists(from) {
            return Err(QDFError::LevelDoesNotExists(from));
        }
        if !self.level_exists(to) {
            return Err(QDFError::LevelDoesNotExists(to));
        }
        if let Some((_, levels)) = astar(&self.graph, from, |f| f == to, |_| 0, |_| 0) {
            Ok(levels)
        } else {
            Ok(vec![])
        }
    }

    /// Updates LOD states.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 1, 16);
    /// let id = {
    ///     let level = lod.level(lod.root()).clone();
    ///     let subs = level.data().as_sublevels();
    ///     let level = lod.level(subs[0]).clone();
    ///     level.data().as_field()
    /// };
    /// {
    ///     let mut qdf = lod.field_mut(id);
    ///     let id = qdf.root();
    ///     qdf.set_space_state(id, 8);
    /// }
    /// lod.recalculate_state();
    /// assert_eq!(*lod.state(), 20);
    /// ```
    pub fn recalculate_state(&mut self) -> Result<S> {
        let id = self.root;
        self.recalculate_level_state(id)
    }

    fn recalculate_level_state(&mut self, id: ID) -> Result<S> {
        if !self.level_exists(id) {
            return Err(QDFError::LevelDoesNotExists(id));
        }
        let mut level = self.levels[&id].clone();
        let state = match level.data() {
            LevelData::SubLevels(sublevels) => {
                let states = sublevels
                    .iter()
                    .map(|l| self.recalculate_level_state(*l))
                    .collect::<Result<Vec<S>>>()?;
                State::merge(&states)
            },
            LevelData::Field(field) => self.fields[field].state().clone(),
        };
        level.apply_state(state.clone());
        self.levels.insert(id, level);
        Ok(state)
    }

    fn subdivide_level(
        id: ID,
        graph: &mut UnGraphMap<ID, ()>,
        levels: &mut HashMap<ID, Level<S>>,
        fields: &mut HashMap<ID, QDF<S>>,
        subdivisions: usize,
        count: usize,
    ) {
        let mut level = levels[&id].clone();
        if level.level() < count {
            let substate = level.state().subdivide(subdivisions);
            let sublevels = (0..subdivisions)
                .map(|idx| {
                    let i = ID::new();
                    graph.add_node(i);
                    Level::new(i, Some(id), level.level() + 1, idx, substate.clone())
                })
                .collect::<Vec<Level<S>>>();
            let first = sublevels[0].id();
            for l in sublevels.iter().skip(1) {
                graph.add_edge(first, l.id(), ());
            }
            level.apply_data(LevelData::SubLevels(sublevels.iter().map(|l| l.id()).collect()));
            for l in sublevels {
                let i = l.id();
                levels.insert(i, l);
                Self::subdivide_level(i, graph, levels, fields, subdivisions, count);
            }
        } else {
            let qdf = QDF::new(subdivisions - 2, level.state().clone());
            level.apply_data(LevelData::Field(qdf.id()));
            fields.insert(qdf.id(), qdf);
        }
        levels.insert(id, level);
    }

    fn connect_clusters(id: ID, graph: &mut UnGraphMap<ID, ()>, levels: &HashMap<ID, Level<S>>) {
        let level = levels[&id].clone();
        if let LevelData::SubLevels(sublevels) = level.data() {
            let neighbors = graph
                .neighbors(id)
                .map(|i| (i, levels[&i].index()))
                .collect::<Vec<(ID, usize)>>();
            for (i, l) in sublevels.iter().enumerate().skip(1) {
                for (nl, ni) in neighbors.iter() {
                    if i != *ni {
                        graph.add_edge(*l, levels[&nl].data().as_sublevels()[i], ());
                    }
                }
            }
            for l in sublevels.iter().skip(1) {
                Self::connect_clusters(*l, graph, levels);
            }
        }
    }
}
