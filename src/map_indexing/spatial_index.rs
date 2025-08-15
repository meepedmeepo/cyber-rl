use std::{
    collections::{HashMap, HashSet},
    os::fd,
    sync::{LazyLock, Mutex},
};

use bracket_lib::prelude::{Algorithm2D, DijkstraMap, Point};
use hecs::Entity;

use crate::{
    Position, State,
    components::{FoV, HasMoved},
    maps::TileType,
};

use super::TileBlocked;

pub static PLAYER_CHASE_MAP: LazyLock<Mutex<DijkstraMap>> =
    LazyLock::new(|| Mutex::new(bracket_lib::pathfinding::DijkstraMap::new_empty(1, 1, 20.0)));

pub static SPATIAL_INDEX: LazyLock<Mutex<SpatialIndexMap>> = LazyLock::new(|| {
    Mutex::new({
        SpatialIndexMap {
            blocked: Vec::new(),
            tile_content: Vec::new(),
            props: HashMap::new(),
            map_height: 0,
            map_width: 0,
        }
    })
});

pub struct SpatialIndexMap {
    blocked: Vec<TileBlocked>,
    tile_content: Vec<Vec<Entity>>,
    props: HashMap<i32, Entity>,
    map_width: usize,
    map_height: usize,
}

impl SpatialIndexMap {
    ///Runs a provided function on each entity that is within provided tile
    pub fn for_each_tile_content<F>(&self, idx: usize, state: &mut State, mut f: F)
    where
        F: FnMut(Entity, &mut State),
    {
        self.tile_content[idx].iter().for_each(|ent| f(*ent, state));
    }

    pub fn get_tile_content(&self, idx: usize) -> std::slice::Iter<Entity> {
        self.tile_content[idx].iter()
    }

    ///Returns list of entities in tile that matches a provided predicate
    pub fn filter_tile_content<F>(&self, idx: usize, state: &State, f: F) -> Vec<Entity>
    where
        F: Fn(Entity, &State) -> bool,
    {
        self.tile_content[idx]
            .iter()
            .filter(|ent| f(**ent, state))
            .map(|ent| *ent)
            .collect::<Vec<Entity>>()
    }

    pub fn is_tile_blocked(&self, idx: usize) -> bool {
        self.blocked[idx].is_blocked()
    }

    pub fn get_tile_contents(&self, idx: usize) -> Vec<Entity> {
        self.tile_content[idx].iter().map(|ent| *ent).collect()
    }
    ///Handles all the steps of correctly carrying out an entity movement action
    pub fn move_entity(&mut self, entity: Entity, entity_movement: Movement, state: &mut State) {
        self.blocked[entity_movement.old_pos].set_ent_block(false);
        self.blocked[entity_movement.new_pos].set_ent_block(true);

        let pos = state.map.index_to_point2d(entity_movement.new_pos);
        *state.world.query_one_mut::<&mut Position>(entity).unwrap() = pos.into();

        state.world.get::<&mut FoV>(entity).unwrap().dirty = true;
        let _ = state.world.insert_one(entity, HasMoved {});
    }

    ///Resets the spatial index and populates which tiles are blocked by a map with a hashset of all blocked map tiles.
    pub fn reset(&mut self, tile_blocked: HashSet<usize>) {
        //Sets whether or not the map blocks a tile.
        self.blocked.iter_mut().enumerate().for_each(|(i, b)| {
            if tile_blocked.contains(&i) {
                *b = TileBlocked {
                    blocked_by_ent: false,
                    blocked_by_map: true,
                };
            } else {
                *b = TileBlocked::default();
            }
        });

        //clear all the tile content vectors
        self.tile_content
            .iter_mut()
            .for_each(|content| content.clear());

        self.props.clear();
    }

    ///Changes current dimensions of the spatial index map.
    ///Call this whenever moving to a different floor.
    pub fn resize<D>(&mut self, d: D)
    where
        D: Into<(usize, usize)>,
    {
        let dimensions = d.into();

        self.map_width = dimensions.0;
        self.map_height = dimensions.1;

        self.blocked = vec![TileBlocked::default(); self.map_width * self.map_width];
        self.tile_content = vec![Vec::new(); self.map_width * self.map_height];
    }

    pub fn add_tile_content(&mut self, idx: usize, entity: Entity) {
        self.tile_content[idx].push(entity);
    }

    pub fn insert_prop(&mut self, idx: i32, entity: Entity) {
        self.props.insert(idx, entity);
    }

    pub fn set_tile_blocked_by_entity(&mut self, idx: usize) {
        self.blocked[idx].set_ent_block(true);
    }

    pub fn set_tile_unblocked_by_entity(&mut self, idx: usize) {
        self.blocked[idx].set_ent_block(false);
    }

    pub fn get_prop_entity_at_idx(&self, idx: i32) -> Option<Entity> {
        self.props.get(&idx).cloned()
    }

    pub fn get_width(&self) -> usize {
        self.map_width
    }

    pub fn get_height(&self) -> usize {
        self.map_height
    }
}

pub struct Movement {
    pub old_pos: usize,
    pub new_pos: usize,
}
