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

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn root(&self) -> ID {
        self.root
    }

    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    #[inline]
    pub fn state(&self) -> &S {
        self.spaces[&self.root].state()
    }

    #[inline]
    pub fn space_exists(&self, id: ID) -> bool {
        self.spaces.contains_key(&id)
    }

    #[inline]
    pub fn try_get_space(&self, id: ID) -> Option<&Space<S>> {
        self.spaces.get(&id)
    }

    #[inline]
    pub fn get_space(&self, id: ID) -> Result<&Space<S>> {
        if let Some(space) = self.spaces.get(&id) {
            Ok(space)
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

    #[inline]
    pub fn space(&self, id: ID) -> &Space<S> {
        &self.spaces[&id]
    }

    #[inline]
    pub fn try_set_space_state(&mut self, id: ID, state: S) -> bool {
        self.set_space_state(id, state).is_ok()
    }

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

    #[inline]
    pub fn find_space_neighbors(&self, id: ID) -> Result<Vec<ID>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::SpaceDoesNotExists(id))
        }
    }

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
