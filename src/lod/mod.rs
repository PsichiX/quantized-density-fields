pub mod level;

pub use self::level::*;
use id::*;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use qdf::QDF;
use qdf::state::State;

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
        let main = Level::new(root, None, 0, state);
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
    pub fn root(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    #[inline]
    pub fn levels_count(&self) -> usize {
        self.count
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
                .map(|_| {
                    let i = ID::new();
                    graph.add_node(i);
                    Level::new(i, Some(id), level.level() + 1, substate.clone())
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
            for l in sublevels {
                Self::connect_clusters(*l, graph, levels);
            }
            // TODO: figure out how to find proper indices in neighbors to use them to connect into.
            // let neighbors = graph.neighbors(id).collect();
        }
    }
}
