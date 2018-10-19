mod tests;
pub mod space;
pub mod state;

use petgraph::graphmap::UnGraphMap;
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
    dimensions: usize,
}

impl<S> QDF<S> where S: State {
    pub fn new(dimensions: usize, root_state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut spaces = HashMap::new();
        let id = Id::new();
        graph.add_node(id);
        spaces.insert(id, Space::with_id(id, root_state));
        Self {
            graph,
            spaces,
            root: id,
            dimensions,
        }
    }

    #[inline]
    pub fn root(&self) -> Id {
        self.root
    }

    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
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

    pub fn find_space_neighbors(&self, id: Id) -> Result<Vec<Id>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    pub fn increase_space_density(&mut self, id: Id) -> Result<()> {
        if self.space_exists(id) {
            let mut space = self.spaces[&id].clone();
            if space.subspace().len() > 0 {
                for s in space.subspace() {
                    self.increase_space_density(*s)?;
                }
            } else {
                let subs = self.dimensions + 1;
                let substate = space.state().subdivide(subs);
                let spaces = (0..subs)
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

    pub fn decrease_space_density(&mut self, id: Id) -> Result<()> {
        if self.space_exists(id) {
            let mut space = self.spaces[&id].clone();
            if space.subspace().is_empty() {
                Err(QDFError::SpaceIsNotSubdivided(id))
            } else {
                // let substate = space.state().subdivide(subs);
                // let spaces = (0..subs)
                // .map(|_| Space::with_id_parent_state(Id::new(), id, substate.clone()))
                // .collect::<Vec<Space<S>>>();
                // let subspace = spaces.iter().map(|s| s.id()).collect::<Vec<Id>>();
                //
                // for s in spaces {
                //     let id = s.id();
                //     self.spaces.insert(id, s);
                //     self.graph.add_node(id);
                // }
                // for a in &subspace {
                //     for b in &subspace {
                //         if a != b {
                //             self.graph.add_edge(*a, *b, ());
                //         }
                //     }
                // }
                // let neighbors = self.graph.neighbors(id).collect::<Vec<Id>>();
                // for (i, n) in neighbors.iter().enumerate() {
                //     self.graph.remove_edge(*n, id);
                //     self.graph.add_edge(*n, subspace[i], ());
                // }
                //
                // space.apply_subspace(subspace);
                // self.spaces.insert(id, space);
                Ok(())
            }
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }
}
