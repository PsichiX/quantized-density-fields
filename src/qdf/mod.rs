mod tests;
pub mod space;
pub mod state;

use petgraph::graphmap::UnGraphMap;
use petgraph::algo::astar;
pub use self::space::*;
pub use self::state::*;
use id::*;
use error::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct QDF<S> where S: State {
    graph: UnGraphMap<Id, ()>,
    spaces: HashMap<Id, Space<S>>,
    root: Id,
    subdivisions: usize,
}

impl<S> QDF<S> where S: State {
    pub fn new(subdivisions: usize, root_state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut spaces = HashMap::new();
        let id = Id::new();
        graph.add_node(id);
        spaces.insert(id, Space::with_id(id, root_state));
        Self {
            graph,
            spaces,
            root: id,
            subdivisions,
        }
    }

    #[inline]
    pub fn root(&self) -> Id {
        self.root
    }

    #[inline]
    pub fn subdivisions(&self) -> usize {
        self.subdivisions
    }

    #[inline]
    pub fn space_exists(&self, id: Id) -> bool {
        self.spaces.contains_key(&id)
    }

    #[inline]
    pub fn try_get_space(&self, id: Id) -> Option<&Space<S>> {
        self.spaces.get(&id)
    }

    #[inline]
    pub fn get_space(&self, id: Id) -> Result<&Space<S>> {
        if let Some(space) = self.spaces.get(&id) {
            Ok(space)
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    #[inline]
    pub fn space(&self, id: Id) -> &Space<S> {
        self.spaces.get(&id).unwrap()
    }

    #[inline]
    pub fn try_set_space_state(&mut self, id: Id, state: S) -> bool {
        self.set_space_state(id, state).is_ok()
    }

    #[inline]
    pub fn set_space_state(&mut self, id: Id, state: S) -> Result<()> {
        if self.space_exists(id) {
            let substate = state.subdivide(self.subdivisions);
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

    pub fn find_space_neighbors(&self, id: Id) -> Result<Vec<Id>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    pub fn find_path(&self, from: Id, to: Id) -> Result<Vec<Id>> {
        if !self.space_exists(from) {
            return Err(QDFError::SpaceDoesNotExists(from));
        }
        if !self.space_exists(to) {
            return Err(QDFError::SpaceDoesNotExists(to));
        }
        if let Some((_, spaces)) = astar(&self.graph, from, |f| f == to, |_| 1, |_| 0) {
            Ok(spaces)
        } else {
            Ok(vec![])
        }
    }

    pub fn increase_space_density(&mut self, id: Id) -> Result<()> {
        if self.space_exists(id) {
            let mut space = self.spaces[&id].clone();
            if !space.is_platonic() {
                for s in space.subspace() {
                    self.increase_space_density(*s)?;
                }
            } else {
                let substate = space.state().subdivide(self.subdivisions);
                let spaces = (0..self.subdivisions)
                    .map(|_| Space::with_id_parent_state(Id::new(), id, substate.clone()))
                    .collect::<Vec<Space<S>>>();
                let subspace = spaces.iter().map(|s| s.id()).collect::<Vec<Id>>();

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
                let neighbors = self.graph.neighbors(id).collect::<Vec<Id>>();
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

    pub fn decrease_space_density(&mut self, id: Id) -> Result<bool> {
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
                    })
                    .collect::<Result<Vec<bool>>>()?
                    .iter()
                    .all(|v| *v);
                if merge {
                    let neighbors = space
                        .subspace()
                        .iter()
                        .flat_map(|s| self.graph.neighbors(*s).collect::<Vec<Id>>())
                        .filter(|s| !space.subspace().contains(s))
                        .collect::<Vec<Id>>();
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

    #[inline]
    pub fn decrease_space_density_level(&mut self, id: Id) -> Result<()> {
        while !self.decrease_space_density(id)? {}
        Ok(())
    }

    fn recalculate_state(&mut self, id: Id) -> Option<Id> {
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
