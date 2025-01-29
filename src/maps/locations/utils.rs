use std::collections::HashSet;

use bracket_lib::prelude::Point;

use crate::maps::BuilderMap;
use petgraph::{self as pg, prelude::GraphMap, visit::Visitable, EdgeType};


pub fn find_entity_spawn_locations(build_data : &BuilderMap) -> HashSet<usize>
{
    let used_locations = build_data.spawn_list.iter().map(|(a,_b)| {
        *a
    }).collect::<HashSet<usize>>();

    build_data.map.map.iter().enumerate()
        .filter_map(|(b,_a)| 
        {
            //build_data.spawn_list.
            if !build_data.map.blocked[b] && !used_locations.contains(&b)
            {
                Some(b)
            }
            else {
                None
            }
        }).collect::<HashSet<usize>>()
}




pub fn create_road_network(build_data : &BuilderMap, start_pos: Point)
{
    let w = build_data.map.map_width;
    let h = build_data.map.map_height;

    let start_idx = build_data.map.xy_idx(start_pos.x, start_pos.y);
    let right_idx = start_idx+1;
    let down_idx = start_idx + w as usize;

    const MIN_SEG_LEN : i32 = 4;
    const ITERATION_LIM : i32 = 20;

    let mut epoch = 0;
    
    let mut graph: GraphMap<usize, i32, petgraph::Undirected> = pg::graphmap::GraphMap::new();

    graph.add_node(start_idx);
    graph.add_node(right_idx);
    graph.add_node(down_idx);

    graph.add_edge(start_idx, right_idx, 0);
    graph.add_edge(start_idx, down_idx, 0);

    

    


}