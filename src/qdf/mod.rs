pub mod space;
pub mod state;
mod tests;

pub use self::space::*;
pub use self::state::*;
use error::*;
use id::*;
use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;

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
    graph: UnGraphMap<ID, ()>,
    spaces: HashMap<ID, Space<S>>,
    root: ID,
    dimensions: usize,
}

impl<S> QDF<S>
where
    S: State,
{
    /// Creates new QDF information universe.
    ///
    /// # Arguments
    /// * `dimensions` - Number of dimensions which space contains.
    /// * `root_state` - State of root space.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// // Creates 2d space with `16` as root state.
    /// let qdf = QDF::new(2, 9);
    /// assert_eq!(*qdf.state(), 9);
    /// ```
    pub fn new(dimensions: usize, root_state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut spaces = HashMap::new();
        let id = ID::new();
        graph.add_node(id);
        spaces.insert(id, Space::with_id(id, root_state));
        Self {
            id: ID::new(),
            graph,
            spaces,
            root: id,
            dimensions,
        }
    }

    /// Gets QDF id.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets QDF root space node id.
    #[inline]
    pub fn root(&self) -> ID {
        self.root
    }

    /// Gets QDF dimensions number.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// assert_eq!(qdf.dimensions(), 2);
    /// ```
    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    /// Gets QDF dimensions number.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// assert_eq!(*qdf.state(), 9);
    /// ```
    #[inline]
    pub fn state(&self) -> &S {
        self.spaces[&self.root].state()
    }

    /// Tells if space with given id exists in QDF.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// assert!(qdf.space_exists(qdf.root()));
    /// ```
    #[inline]
    pub fn space_exists(&self, id: ID) -> bool {
        self.spaces.contains_key(&id)
    }

    /// Try to get given space.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// if let Some(space) = qdf.try_get_space(qdf.root()) {
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// if let Ok(space) = qdf.get_space(qdf.root()) {
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let qdf = QDF::new(2, 9);
    /// assert_eq!(*qdf.space(qdf.root()).state(), 9);
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// assert!(qdf.try_set_space_state(id, 3));
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// assert!(qdf.set_space_state(id, 3).is_ok());
    /// ```
    #[inline]
    pub fn set_space_state(&mut self, id: ID, state: S) -> Result<()> {
        if self.space_exists(id) {
            let substate = state.subdivide(self.dimensions + 1);
            let mut space = self.spaces[&id].clone();
            space.apply_state(state);
            for s in space.subspace() {
                self.set_space_state(*s, substate.clone())?;
            }
            let mut parent = space.parent();
            self.spaces.insert(id, space);
            while parent.is_some() {
                parent = self.recalculate_state(parent.unwrap());
            }
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// qdf.increase_space_density(id);
    /// let subs = qdf.space(qdf.root()).subspace();
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
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// qdf.increase_space_density(id);
    /// let subs = qdf.space(qdf.root()).subspace().to_vec();
    /// qdf.increase_space_density(subs[0]);
    /// let subs2 = qdf.space(subs[0]).subspace();
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
    /// or throws error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// qdf.increase_space_density(id);
    /// assert_eq!(qdf.space(qdf.root()).subspace().len(), 3);
    /// ```
    pub fn increase_space_density(&mut self, id: ID) -> Result<()> {
        if self.space_exists(id) {
            let mut space = self.spaces[&id].clone();
            if !space.is_platonic() {
                for s in space.subspace() {
                    self.increase_space_density(*s)?;
                }
            } else {
                let subs = self.dimensions + 1;
                let substate = space.state().subdivide(subs);
                let spaces = (0..subs)
                    .map(|_| Space::with_id_parent_state(ID::new(), id, substate.clone()))
                    .collect::<Vec<Space<S>>>();
                let subspace = spaces.iter().map(|s| s.id()).collect::<Vec<ID>>();

                for s in spaces {
                    let id = s.id();
                    self.spaces.insert(id, s);
                    self.graph.add_node(id);
                }
                for a in &subspace {
                    for b in &subspace {
                        if a != b {
                            self.graph.add_edge(*a, *b, ());
                        }
                    }
                }
                let neighbors = self.graph.neighbors(id).collect::<Vec<ID>>();
                for (i, n) in neighbors.iter().enumerate() {
                    self.graph.remove_edge(*n, id);
                    self.graph.add_edge(*n, subspace[i], ());
                }

                space.apply_subspace(subspace);
                self.spaces.insert(id, space);
            }
            Ok(())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Decreases given space density (merge space children and rebind them properly to theirs
    /// neighbors if space has 1 level of subdivision, otherwise perform this operation on its
    /// subspaces), or throws error if space does not exists.
    ///
    /// # Arguments
    /// * `id` - space id.
    ///
    /// # Examples
    /// ```
    /// use quantized_density_fields::QDF;
    ///
    /// let mut qdf = QDF::new(2, 9);
    /// let id = qdf.root();
    /// qdf.increase_space_density(id);
    /// assert_eq!(qdf.space(qdf.root()).subspace().len(), 3);
    /// qdf.decrease_space_density(id);
    /// assert!(qdf.space(qdf.root()).is_platonic());
    /// ```
    pub fn decrease_space_density(&mut self, id: ID) -> Result<bool> {
        if self.space_exists(id) {
            let mut space = self.spaces[&id].clone();
            if space.is_platonic() {
                Ok(true)
            } else {
                let merge = space
                    .subspace()
                    .iter()
                    .map(|id| {
                        if self.spaces[id].is_platonic() {
                            Ok(true)
                        } else {
                            self.decrease_space_density(*id)
                        }
                    }).collect::<Result<Vec<bool>>>()?
                    .iter()
                    .all(|v| *v);
                if merge {
                    let neighbors = space
                        .subspace()
                        .iter()
                        .flat_map(|s| self.graph.neighbors(*s).collect::<Vec<ID>>())
                        .filter(|s| !space.subspace().contains(s))
                        .collect::<Vec<ID>>();
                    for n in neighbors {
                        self.graph.add_edge(id, n, ());
                    }
                    for s in space.subspace() {
                        self.graph.remove_node(*s);
                        self.spaces.remove(s);
                    }
                    space.apply_subspace(vec![]);
                    self.spaces.insert(id, space);
                }
                Ok(false)
            }
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    /// Decreases given space density (merge space children and rebind them properly to theirs
    /// neighbors), or throws error if space does not exists. Basically it works like
    /// `Self::decrease_space_density()` but merges space to make it completely platonic.
    ///
    /// # Arguments
    /// * `id` - space id.
    #[inline]
    pub fn decrease_space_density_level(&mut self, id: ID) -> Result<()> {
        while !self.decrease_space_density(id)? {}
        Ok(())
    }

    fn recalculate_state(&mut self, id: ID) -> Option<ID> {
        let mut space = self.spaces[&id].clone();
        let states = space
            .subspace()
            .iter()
            .map(|s| self.spaces[&s].state().clone())
            .collect::<Vec<S>>();
        space.apply_state(Subdividable::merge(&states));
        let parent = space.parent();
        self.spaces.insert(id, space);
        parent
    }
}
