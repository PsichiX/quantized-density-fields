pub mod level;
mod tests;

pub use self::level::*;
use error::*;
use id::*;
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::astar;
use std::collections::HashMap;
use qdf::*;

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
    pub fn new(dimensions: usize, count: usize, state: S) -> Self {
        let mut graph = UnGraphMap::new();
        let mut levels = HashMap::new();
        let mut fields = HashMap::new();
        let root = ID::new();
        let main = Level::new(root, None, 0, 0, state);
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
    pub fn levels_count(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn state(&self) -> &S {
        self.levels[&self.root].state()
    }

    #[inline]
    pub fn level_exists(&self, id: ID) -> bool {
        self.levels.contains_key(&id)
    }

    #[inline]
    pub fn try_get_level(&self, id: ID) -> Option<&Level<S>> {
        self.levels.get(&id)
    }

    #[inline]
    pub fn get_level(&self, id: ID) -> Result<&Level<S>> {
        if let Some(level) = self.levels.get(&id) {
            Ok(level)
        } else {
            Err(QDFError::LevelDoesNotExists(id))
        }
    }

    #[inline]
    pub fn level(&self, id: ID) -> &Level<S> {
        &self.levels[&id]
    }

    #[inline]
    pub fn field_exists(&self, id: ID) -> bool {
        self.fields.contains_key(&id)
    }

    #[inline]
    pub fn try_get_field(&self, id: ID) -> Option<&QDF<S>> {
        self.fields.get(&id)
    }

    #[inline]
    pub fn try_get_field_mut(&mut self, id: ID) -> Option<&mut QDF<S>> {
        self.fields.get_mut(&id)
    }

    #[inline]
    pub fn get_field(&self, id: ID) -> Result<&QDF<S>> {
        if let Some(field) = self.fields.get(&id) {
            Ok(field)
        } else {
            Err(QDFError::FieldDoesNotExists(id))
        }
    }

    #[inline]
    pub fn get_field_mut(&mut self, id: ID) -> Result<&mut QDF<S>> {
        if let Some(field) = self.fields.get_mut(&id) {
            Ok(field)
        } else {
            Err(QDFError::FieldDoesNotExists(id))
        }
    }

    #[inline]
    pub fn field(&self, id: ID) -> &QDF<S> {
        &self.fields[&id]
    }

    #[inline]
    pub fn field_mut(&mut self, id: ID) -> &mut QDF<S> {
        self.fields.get_mut(&id).unwrap()
    }

    #[inline]
    pub fn find_level_neighbors(&self, id: ID) -> Result<Vec<ID>> {
        if self.graph.contains_node(id) {
            Ok(self.graph.neighbors(id).collect())
        } else {
            Err(QDFError::LevelDoesNotExists(id))
        }
    }

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

    pub fn recalculate_level_state(&mut self, id: ID) -> Result<S> {
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
                Subdividable::merge(&states)
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
