pub mod simulate;
pub mod space;
pub mod state;
mod tests;

pub use self::simulate::*;
pub use self::space::*;
pub use self::state::*;
use error::*;
use id::*;
use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;
use rayon::prelude::*;
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};

/// Short hand type alias for space graph.
pub type SpaceGraph = UnGraphMap<ID, ()>;
/// Short hand type alias for space map.
pub type SpaceMap<S> = HashMap<ID, Space<S>>;

/// Object that represents quantized density fields.
///
/// # Concept
/// QDF does not exists in any space - it IS the space, it defines it,
/// it describes it so there are no space coordinates and it is your responsibility to deliver it.
/// In future releases this crate will have module for projecting QDF into Euclidean space
/// and will have a satelite crate to easlyy traverse and visualize space.
///
/// To sample specified region you have to know some space ID and gather the rest of information
/// based on it neighbors spaces.
/// It gives the ability to cotrol space density at specified locations, which can be used
/// for example to simulate space curvature based on gravity.
#[derive(Debug)]
pub struct QDF<S>
where
    S: State,
{
    id: ID,
    graph: SpaceGraph,
    spaces: SpaceMap<S>,
    space_ids: HashSet<ID>,
    dimensions: usize,
}

impl<S> QDF<S>
where
    S: State,
{
    /// Creates new QDF information universe.
    ///
    /// # Arguments
    /// * `dimensions` - Number of dimensions space contains.
    /// * `state` - State of space.
    ///
    /// # Returns
    /// Tuple of new QDF object and space id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// // Creates 2d space with `9` as root state.
    /// let (qdf, root) = QDF::new(2, 9);
    /// assert_eq!(*qdf.space(root).state(), 9);
    /// ```
    pub fn new(dimensions: usize, state: S) -> (Self, ID) {
        let mut graph = UnGraphMap::new();
        let mut spaces = HashMap::new();
        let mut space_ids = HashSet::new();
        let id = ID::new();
        graph.add_node(id);
        spaces.insert(id, Space::new(id, state));
        space_ids.insert(id);
        let qdf = Self {
            id: ID::new(),
            graph,
            spaces,
            space_ids,
            dimensions,
        };
        (qdf, id)
    }

    /// Creates new QDF information universe and increase its levels of density.
    ///
    /// # Arguments
    /// * `dimensions` - Number of dimensions which space contains.
    /// * `state` - State of space.
    /// * `levels` - Number of levels of uniform density.
    ///
    /// # Returns
    /// Tuple of new QDF object and vector of space ids.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::{QDF, State};
    ///
    /// // Creates 2d space with `27` as root state and 2 levels of uniform density.
    /// let (qdf, spaces) = QDF::with_levels(2, 27, 2);
    /// assert_eq!(spaces.len(), (qdf.dimensions() + 1).pow(2));
    /// assert_eq!(*qdf.space(spaces[0]).state(), 3);
    /// assert_eq!(
    ///     State::merge(&qdf.spaces().map(|id| *qdf.space(*id).state()).collect::<Vec<i32>>()),
    ///     27,
    /// );
    /// ```
    pub fn with_levels(dimensions: usize, state: S, levels: usize) -> (Self, Vec<ID>) {
        let (mut qdf, _) = Self::new(dimensions, state);
        for _ in 0..levels {
            let spaces = qdf.spaces().cloned().collect::<Vec<ID>>();
            for id in spaces {
                qdf.increase_space_density(id).unwrap();
            }
        }
        let spaces = qdf.spaces().cloned().collect();
        (qdf, spaces)
    }

    /// Creates new QDF information universe and increase its levels of density and state applied
    /// to lowest space lavel.
    ///
    /// # Arguments
    /// * `dimensions` - Number of dimensions which space contains.
    /// * `state` - State of space at lowest level.
    /// * `levels` - Number of levels of uniform density.
    ///
    /// # Returns
    /// Tuple of new QDF object and vector of space ids.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::{QDF, State};
    ///
    /// // Creates 2d space with `3` as lowest level state and 2 levels of uniform density.
    /// let (qdf, spaces) = QDF::with_levels_and_minimum_state(2, 3, 2);
    /// assert_eq!(spaces.len(), (qdf.dimensions() + 1).pow(2));
    /// assert_eq!(*qdf.space(spaces[0]).state(), 3);
    /// assert_eq!(
    ///     State::merge(&qdf.spaces().map(|id| *qdf.space(*id).state()).collect::<Vec<i32>>()),
    ///     27,
    /// );
    /// ```
    #[inline]
    pub fn with_levels_and_minimum_state(
        dimensions: usize,
        state: S,
        levels: usize,
    ) -> (Self, Vec<ID>) {
        Self::with_levels(dimensions, state.super_state_at_level(dimensions, levels), levels)
    }

    /// Gets QDF id.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets QDF dimensions number.
    ///
    /// # Returns
    /// Number of dimensions (axes) space has.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (qdf, _) = QDF::new(2, 9);
    /// assert_eq!(qdf.dimensions(), 2);
    /// ```
    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    /// Tells if space with given id exists in QDF.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `true` if given space exists, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (qdf, root) = QDF::new(2, 9);
    /// assert!(qdf.space_exists(root));
    /// ```
    #[inline]
    pub fn space_exists(&self, id: ID) -> bool {
        self.spaces.contains_key(&id)
    }

    /// Gets iterator over all spaces IDs.
    ///
    /// # Returns
    /// Iterator over all space ids.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::{QDF, ID};
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// assert_eq!(qdf.spaces().count(), 1);
    /// assert_eq!(*qdf.spaces().nth(0).unwrap(), root);
    /// let (_, mut subs, _) = qdf.increase_space_density(root).unwrap();
    /// subs.sort();
    /// assert_eq!(qdf.spaces().count(), 3);
    /// let mut spaces = qdf.spaces().cloned().collect::<Vec<ID>>();
    /// spaces.sort();
    /// assert_eq!(spaces, subs);
    /// ```
    #[inline]
    pub fn spaces(&self) -> Iter<ID> {
        self.space_ids.iter()
    }

    /// Try to get given space.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `Some` reference to given `Space` data or `None` if space does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (qdf, root) = QDF::new(2, 9);
    /// if let Some(space) = qdf.try_get_space(root) {
    ///     assert_eq!(*space.state(), 9);
    /// }
    /// ```
    #[inline]
    pub fn try_get_space(&self, id: ID) -> Option<&Space<S>> {
        self.spaces.get(&id)
    }

    /// Get given space or throw error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `Ok` with reference to given `Space` data or `Err` if space does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (qdf, root) = QDF::new(2, 9);
    /// if let Ok(space) = qdf.get_space(root) {
    ///     assert_eq!(*space.state(), 9);
    /// }
    /// ```
    #[inline]
    pub fn get_space(&self, id: ID) -> Result<&Space<S>> {
        if let Some(space) = self.spaces.get(&id) {
            Ok(space)
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Get given space or panic if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// Reference to `Space` data.
    ///
    /// # Panics
    /// When given space does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (qdf, root) = QDF::new(2, 9);
    /// assert_eq!(*qdf.space(root).state(), 9);
    /// ```
    #[inline]
    pub fn space(&self, id: ID) -> &Space<S> {
        &self.spaces[&id]
    }

    /// Try to set given space state.
    ///
    /// # Arguments
    /// * `id` - space id.
    /// * `state` - state.
    ///
    /// # Returns
    /// `true` if space exists and state was successfuly set, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// assert!(qdf.try_set_space_state(root, 3));
    /// ```
    #[inline]
    pub fn try_set_space_state(&mut self, id: ID, state: S) -> bool {
        self.set_space_state(id, state).is_ok()
    }

    /// Set given space state or throw error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    /// * `state` - state.
    ///
    /// # Returns
    /// `Ok` if space exists and state was successfuly set, `Err` otherwise.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// assert!(qdf.set_space_state(root, 3).is_ok());
    /// ```
    #[inline]
    pub fn set_space_state(&mut self, id: ID, state: S) -> Result<()> {
        if self.space_exists(id) {
            self.spaces.get_mut(&id).unwrap().apply_state(state);
            Ok(())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Get list of IDs of given space neighbors or throws error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `Ok` with vector of space neighbors if space exists, `Err` otherwise.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// let (_, subs, _) = qdf.increase_space_density(root).unwrap();
    /// assert_eq!(qdf.find_space_neighbors(subs[0]).unwrap(), vec![subs[1], subs[2]]);
    /// ```
    #[inline]
    pub fn find_space_neighbors(&self, id: ID) -> Result<Vec<ID>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Gets list of space IDs that defines shortest path between two spaces,
    /// or throws error if space does not exists.
    ///
    /// # Arguments
    /// * `from` - source space id.
    /// * `to` - target space id.
    ///
    /// # Returns
    /// `Ok` with space ids that builds shortest path between two points, `Err` if path cannot be
    /// found or spaces does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// let (_, subs, _) = qdf.increase_space_density(root).unwrap();
    /// let (_, subs2, _) = qdf.increase_space_density(subs[0]).unwrap();
    /// assert_eq!(qdf.find_path(subs2[0], subs[2]).unwrap(), vec![subs2[0], subs2[1], subs[2]]);
    /// ```
    pub fn find_path(&self, from: ID, to: ID) -> Result<Vec<ID>> {
        if !self.space_exists(from) {
            return Err(QDFError::SpaceDoesNotExists(from));
        }
        if !self.space_exists(to) {
            return Err(QDFError::SpaceDoesNotExists(to));
        }
        if let Some((_, spaces)) = astar(&self.graph, from, |f| f == to, |_| 0, |_| 0) {
            Ok(spaces)
        } else {
            Ok(vec![])
        }
    }

    /// Increases given space density (subdivide space and rebind it properly to its neighbors),
    /// and returns process information (source space id, subdivided space ids, connections pairs)
    /// or throws error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `Ok` with tuple of source space id, vector of subdivided space ids and vector of
    /// connections pairs or `Err` if space does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// let (_, subs, _) = qdf.increase_space_density(root).unwrap();
    /// assert_eq!(subs.len(), 3);
    /// ```
    pub fn increase_space_density(&mut self, id: ID) -> Result<(ID, Vec<ID>, Vec<(ID, ID)>)> {
        if self.space_exists(id) {
            let space = self.spaces[&id].clone();
            let subs = self.dimensions + 1;
            let substates = space.state().subdivide(subs);
            let spaces = substates
                .iter()
                .map(|substate| Space::new(ID::new(), substate.clone()))
                .collect::<Vec<Space<S>>>();
            for s in &spaces {
                let id = s.id();
                self.spaces.insert(id, s.clone());
                self.graph.add_node(id);
                self.space_ids.insert(id);
            }
            for a in &spaces {
                let aid = a.id();
                for b in &spaces {
                    let bid = b.id();
                    if aid != bid {
                        self.graph.add_edge(aid, bid, ());
                    }
                }
            }
            let neighbors = self.graph.neighbors(id).collect::<Vec<ID>>();
            let pairs = neighbors
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    let t = spaces[i].id();
                    self.graph.remove_edge(*n, id);
                    self.graph.add_edge(*n, t, ());
                    (*n, t)
                })
                .collect::<Vec<(ID, ID)>>();
            self.space_ids.remove(&id);
            self.spaces.remove(&id);
            let space_ids = spaces.iter().map(|s| s.id()).collect::<Vec<ID>>();
            Ok((id, space_ids, pairs))
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Decreases given space density (merge space children and rebind them properly to theirs
    /// neighbors if space has 1 level of subdivision, otherwise perform this operation on its
    /// subspaces), and returns process information (source space ids, merged space id) or throws
    /// error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Returns
    /// `Ok` with `Some` tuple of vector of merged space ids and created space id, or `Ok` with
    /// `None` if space cannot be merged or `Err` if given space does not exists.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let (mut qdf, root) = QDF::new(2, 9);
    /// let (_, subs, _) = qdf.increase_space_density(root).unwrap();
    /// assert_eq!(subs.len(), 3);
    /// let (_, root) = qdf.decrease_space_density(subs[0]).unwrap().unwrap();
    /// assert_eq!(qdf.spaces().len(), 1);
    /// assert_eq!(*qdf.spaces().nth(0).unwrap(), root);
    /// ```
    pub fn decrease_space_density(&mut self, id: ID) -> Result<Option<(Vec<ID>, ID)>> {
        if self.space_exists(id) {
            let neighbor = self.graph.neighbors(id).collect::<Vec<ID>>();
            let mut connected = neighbor
                .iter()
                .filter(|a| {
                    neighbor
                        .iter()
                        .any(|b| **a != *b && self.graph.edge_weight(**a, *b).is_some())
                }).cloned()
                .collect::<Vec<ID>>();
            if connected.len() != self.dimensions {
                Ok(None)
            } else {
                connected.push(id);
                let states = connected
                    .iter()
                    .map(|i| self.spaces[&i].state())
                    .cloned()
                    .collect::<Vec<S>>();
                let id = ID::new();
                self.graph.add_node(id);
                self.space_ids.insert(id);
                self.spaces
                    .insert(id, Space::new(id, State::merge(&states)));
                for i in &connected {
                    let outsiders = self
                        .graph
                        .neighbors(*i)
                        .filter(|n| !connected.contains(n))
                        .collect::<Vec<ID>>();
                    for n in outsiders {
                        self.graph.add_edge(id, n, ());
                    }
                }
                let space_ids = connected
                    .iter()
                    .map(|i| {
                        self.graph.remove_node(*i);
                        self.spaces.remove(i);
                        self.space_ids.remove(i);
                        *i
                    })
                    .collect::<Vec<ID>>();
                Ok(Some((space_ids, id)))
            }
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
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
            self.spaces.get_mut(&id).unwrap().apply_state(state);
        }
    }

    /// Does the same as `simulation_step()` but in parallel manner (it may or may not increase
    /// simulation performance if simulation is very complex).
    pub fn simulation_step_parallel<M>(&mut self)
    where
        M: Simulate<S>,
    {
        let states = self.simulate_states_parallel::<M>();
        for (id, state) in states {
            self.spaces.get_mut(&id).unwrap().apply_state(state);
        }
    }

    /// Performs simulation on QDF like `simulation_step()` but instead of applying results to QDF,
    /// it returns simulated platonic space states along with their space ID.
    ///
    /// # Returns
    /// Vector of tuples of id and its updated space that were simulated.
    pub fn simulate_states<M>(&self) -> Vec<(ID, S)>
    where
        M: Simulate<S>,
    {
        self.space_ids
            .iter()
            .map(|id| {
                let neighbor_states = self
                    .graph
                    .neighbors(*id)
                    .map(|i| self.spaces[&i].state())
                    .collect::<Vec<&S>>();
                (*id, M::simulate(self.spaces[id].state(), &neighbor_states))
            }).collect()
    }

    /// Performs simulation on QDF like `simulation_step_parallel()` but instead of applying
    /// results to QDF, it returns simulated platonic space states along with their space ID.
    ///
    /// # Returns
    /// Vector of tuples of id and its updated space that were simulated.
    pub fn simulate_states_parallel<M>(&self) -> Vec<(ID, S)>
    where
        M: Simulate<S>,
    {
        let spaces = &self.spaces;
        let space_ids = &self.space_ids;
        let graph = &self.graph;
        space_ids
            .par_iter()
            .map(|id| {
                let neighbor_states = graph
                    .neighbors(*id)
                    .map(|i| spaces[&i].state())
                    .collect::<Vec<&S>>();
                (*id, M::simulate(spaces[id].state(), &neighbor_states))
            }).collect()
    }
}
