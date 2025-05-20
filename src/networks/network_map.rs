use std::collections::{HashMap, HashSet};

use bracket_lib::prelude::console;
use hecs::Entity;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NodeIndex {
    pub entity: Entity,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NodeConnection {
    pub entity: Entity,
}
#[derive(Clone, Debug)]
pub struct NetworkMap {
    graph: HashMap<NodeIndex, Vec<NodeConnection>>,
    pub alert: i32,
}

impl NetworkMap {
    pub fn get_connections(&self, ent: Entity) -> Option<&Vec<NodeConnection>> {
        self.graph
            .iter()
            .find(|(index, edges)| NodeIndex { entity: ent } == **index)
            .and_then(|(index, edges)| {
                return Some(edges);
            })
    }

    pub fn add_node(&mut self, ent: Entity) -> bool {
        match self.graph.insert(NodeIndex { entity: ent }, Vec::new()) {
            Some(_) => true,
            None => false,
        }
    }

    fn add_connection(&mut self, node1: Entity, node2: Entity) -> bool {
        self.graph
            .iter_mut()
            .find(|(index, _edges)| **index == NodeIndex { entity: node1 })
            .map_or(false, |(_index, edges)| {
                edges.push(NodeConnection { entity: node2 });
                return true;
            })
    }

    pub fn add_edge(&mut self, node1: Entity, node2: Entity) -> bool {
        if self.graph.len() < 2 {
            console::log("The NetworkMap doesn't have at least 2 edges to try to connect!");
            return false;
        }
        if !self.add_connection(node1, node2) {
            return false;
        }
        if !self.add_connection(node2, node1) {
            return false;
        }

        true
    }

    pub fn for_each(&mut self, f: fn(&NodeIndex, &mut Vec<NodeConnection>)) {
        self.graph
            .iter_mut()
            .for_each(|(index, connection)| f(index, connection));
    }

    ///Tries to create a new NetworkMap with alert level 0 from a given root node
    pub fn new(ent: Entity) -> Result<NetworkMap, ()> {
        let mut map = NetworkMap {
            graph: HashMap::new(),
            alert: 0,
        };

        if map.add_node(ent) {
            Ok(map.clone())
        } else {
            Err(())
        }
    }

    pub fn empty() -> NetworkMap {
        NetworkMap {
            graph: HashMap::new(),
            alert: 0,
        }
    }
}
