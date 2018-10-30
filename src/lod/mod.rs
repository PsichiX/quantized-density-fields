pub mod level;
mod tests;

pub use self::level::*;
use error::*;
use id::*;
use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;
use qdf::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

/// Object that represents space level of details.
/// This gives you the ability to sample space area states at different zoom levels (LOD mechanism).
#[derive(Debug)]
pub struct LOD<S>
where
    S: State,
{
    id: ID,
    graph: UnGraphMap<ID, ()>,
    levels: HashMap<ID, Level<S>>,
    platonic_levels: HashSet<ID>,
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
    /// assert_eq!(lod.level(lod.root()).sublevels().len(), 4);
    /// // sampled state at level 1 equals to `4` (`16 / 4`).
    /// assert_eq!(*lod.level(lod.level(lod.root()).sublevels()[0]).state(), 4);
    /// ```
    pub fn new(dimensions: usize, count: usize, root_state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut levels = HashMap::new();
        let mut platonic_levels = HashSet::new();
        let root = ID::new();
        let main = Level::new(root, None, 0, 0, root_state);
        levels.insert(root, main);
        graph.add_node(root);
        Self::subdivide_level(root, &mut graph, &mut levels, dimensions + 2, count);
        Self::connect_clusters(root, &mut graph, &levels);
        Self::collect_platonic_levels(root, &levels, &mut platonic_levels);
        Self {
            id: ID::new(),
            graph,
            levels,
            platonic_levels,
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

    /// Try to set given level state.
    ///
    /// # Arguments
    /// * `id` - level id.
    /// * `state` - state.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 1, 9);
    /// let id = lod.root();
    /// assert!(lod.try_set_level_state(id, 3));
    /// ```
    #[inline]
    pub fn try_set_level_state(&mut self, id: ID, state: S) -> bool {
        self.set_level_state(id, state).is_ok()
    }

    /// Set given level state or throw error if level does not exists.
    ///
    /// # Arguments
    /// * `id` - level id.
    /// * `state` - state.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::LOD;
    ///
    /// let mut lod = LOD::new(2, 1, 9);
    /// let id = lod.root();
    /// assert!(lod.set_level_state(id, 3).is_ok());
    /// ```
    #[inline]
    pub fn set_level_state(&mut self, id: ID, state: S) -> Result<()> {
        if self.level_exists(id) {
            self.levels.get_mut(&id).unwrap().apply_state(state);
            self.recalculate_children_states(id);
            self.recalculate_parent_state(id);
            Ok(())
        } else {
            Err(QDFError::LevelDoesNotExists(id))
        }
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
    /// let subs = lod.level(lod.root()).sublevels();
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
    /// let subs = lod.level(lod.root()).sublevels();
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

    /// Performs simulation step (go through all platonic spaces and modifies its states based on
    /// neighbor states). Actual state simulation is performed by your struct that implements
    /// `Simulation` trait.
    pub fn simulation_step<M>(&mut self)
    where
        M: Simulate<S>,
    {
        let states = self.simulate_states::<M>();
        for (id, state) in states {
            self.levels.get_mut(&id).unwrap().apply_state(state);
        }
        let root = self.root;
        self.recalculate_states(root);
    }

    /// Does the same as `simulation_step()` but in parallel manner (it may or may not increase
    /// simulation performance if simulation is very complex).
    pub fn simulation_step_parallel<M>(&mut self)
    where
        M: Simulate<S>,
    {
        let states = self.simulate_states_parallel::<M>();
        for (id, state) in states {
            self.levels.get_mut(&id).unwrap().apply_state(state);
        }
        let root = self.root;
        self.recalculate_states(root);
    }

    /// Performs simulation on LOD like `simulation_step()` but instead of applying results to LOD,
    /// it returns simulated platonic level states along with their level ID.
    pub fn simulate_states<M>(&self) -> Vec<(ID, S)>
    where
        M: Simulate<S>,
    {
        self.platonic_levels
            .iter()
            .map(|id| {
                let neighbor_states = self
                    .graph
                    .neighbors(*id)
                    .map(|i| self.levels[&i].state())
                    .collect::<Vec<&S>>();
                (*id, M::simulate(self.levels[id].state(), &neighbor_states))
            }).collect()
    }

    /// Performs simulation on LOD like `simulation_step_parallel()` but instead of applying
    /// results to LOD, it returns simulated platonic level states along with their level ID.
    pub fn simulate_states_parallel<M>(&self) -> Vec<(ID, S)>
    where
        M: Simulate<S>,
    {
        self.platonic_levels
            .par_iter()
            .map(|id| {
                let neighbor_states = self
                    .graph
                    .neighbors(*id)
                    .map(|i| self.levels[&i].state())
                    .collect::<Vec<&S>>();
                (*id, M::simulate(self.levels[id].state(), &neighbor_states))
            }).collect()
    }

    fn subdivide_level(
        id: ID,
        graph: &mut UnGraphMap<ID, ()>,
        levels: &mut HashMap<ID, Level<S>>,
        subdivisions: usize,
        count: usize,
    ) {
        // TODO: optimize!
        let mut level = levels[&id].clone();
        if level.level() < count {
            let substates = level.state().subdivide(subdivisions);
            let sublevels = substates
                .iter()
                .enumerate()
                .map(|(idx, substate)| {
                    let i = ID::new();
                    graph.add_node(i);
                    Level::new(i, Some(id), level.level() + 1, idx, substate.clone())
                }).collect::<Vec<Level<S>>>();
            let first = sublevels[0].id();
            for l in sublevels.iter().skip(1) {
                graph.add_edge(first, l.id(), ());
            }
            level.apply_sublevels(sublevels.iter().map(|l| l.id()).collect());
            for l in sublevels {
                let i = l.id();
                levels.insert(i, l);
                Self::subdivide_level(i, graph, levels, subdivisions, count);
            }
            levels.insert(id, level);
        }
    }

    fn connect_clusters(id: ID, graph: &mut UnGraphMap<ID, ()>, levels: &HashMap<ID, Level<S>>) {
        let sublevels = levels[&id].sublevels();
        if !sublevels.is_empty() {
            let neighbors = graph
                .neighbors(id)
                .map(|i| (i, levels[&i].index()))
                .collect::<Vec<(ID, usize)>>();
            for (i, l) in sublevels.iter().enumerate().skip(1) {
                for (nl, ni) in &neighbors {
                    if i != *ni {
                        graph.add_edge(*l, levels[&nl].sublevels()[i], ());
                    }
                }
            }
            for l in sublevels.iter().skip(1) {
                Self::connect_clusters(*l, graph, levels);
            }
        }
    }

    fn collect_platonic_levels(
        id: ID,
        levels: &HashMap<ID, Level<S>>,
        platonic_levels: &mut HashSet<ID>,
    ) {
        let sublevels = levels[&id].sublevels();
        if sublevels.is_empty() {
            platonic_levels.insert(id);
        } else {
            for id in sublevels {
                Self::collect_platonic_levels(*id, levels, platonic_levels);
            }
        }
    }

    fn recalculate_states(&mut self, id: ID) -> S {
        let level = self.levels[&id].clone();
        if level.sublevels().is_empty() {
            level.state().clone()
        } else {
            let states = level
                .sublevels()
                .iter()
                .map(|i| self.recalculate_states(*i))
                .collect::<Vec<S>>();
            let state = State::merge(&states);
            self.levels.get_mut(&id).unwrap().apply_state(state.clone());
            state
        }
    }

    fn recalculate_children_states(&mut self, id: ID) {
        let level = self.levels[&id].clone();
        let states = level.state().subdivide(self.dimensions + 2);
        for (id, state) in level.sublevels().iter().zip(states.into_iter()) {
            self.levels.get_mut(&id).unwrap().apply_state(state);
            self.recalculate_children_states(*id);
        }
    }

    fn recalculate_parent_state(&mut self, id: ID) {
        if let Some(id) = self.levels[&id].parent() {
            let level = self.levels[&id].clone();
            let states = level
                .sublevels()
                .iter()
                .map(|i| self.levels[i].state().clone())
                .collect::<Vec<S>>();
            self.levels
                .get_mut(&id)
                .unwrap()
                .apply_state(State::merge(&states));
            self.recalculate_parent_state(id);
        }
    }
}
