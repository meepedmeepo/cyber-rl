use bracket_lib::{prelude::{DistanceAlg, Point}, random::RandomNumberGenerator};

use super::{BuilderMap, InitialMapBuilder, Map, TileType, MAPHEIGHT, MAPSIZE, MAPWIDTH};


#[derive(PartialEq, Copy, Clone)]
pub enum DistanceAlgorithm {Pythagoras, Manhattan, Chebyshev}

pub struct VoronoiCellBuilder
{
    n_seeds : usize,
    distance_algorithm : DistanceAlgorithm
}

impl InitialMapBuilder for VoronoiCellBuilder
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        self.build(rng, build_data);
    }
}

impl VoronoiCellBuilder
{
    pub fn new() -> Box<VoronoiCellBuilder>
    {
        Box::new(VoronoiCellBuilder 
                {
                n_seeds: 64
                , distance_algorithm: DistanceAlgorithm::Chebyshev, 
                })
    }
    
    pub fn pythagoras() -> Box<VoronoiCellBuilder>
    {
        Box::new(VoronoiCellBuilder 
            {
            n_seeds: 64
            , distance_algorithm: DistanceAlgorithm::Pythagoras, 
            })
    }

    pub fn manhattan() -> Box<VoronoiCellBuilder>
    {
        Box::new(VoronoiCellBuilder 
            {
            n_seeds: 64
            , distance_algorithm: DistanceAlgorithm::Manhattan, 
            })
    }


    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        //Creates a voronoi diagram
        let mut voronoi_seeds : Vec<(usize, Point)> = Vec::new();

        while voronoi_seeds.len() < self.n_seeds
        {
            let vx = rng.roll_dice(1, MAPWIDTH-1);
            let vy = rng.roll_dice(2, MAPHEIGHT-1);
            let vidx = Map::xy_id(vx, vy);
            let candidate = (vidx, Point::new(vx,vy));

            if !voronoi_seeds.contains(&candidate)
            {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_distance = vec![(0, 0.0f32) ; self.n_seeds];
        let mut voronoi_membership : Vec<i32> = vec![0 ; MAPSIZE];

        for (i, vid) in voronoi_membership.iter_mut().enumerate()
        {
            let x = i as i32 % MAPWIDTH;
            let y = i as i32 / MAPWIDTH;

            for (seed, pos) in voronoi_seeds.iter().enumerate()
            {
                let distance;
                match self.distance_algorithm
                {
                    DistanceAlgorithm::Pythagoras =>
                    {
                        distance = DistanceAlg::PythagorasSquared.distance2d(Point::new(x, y), pos.1);
                    }
                    DistanceAlgorithm::Manhattan =>
                    {
                        distance = DistanceAlg::Manhattan.distance2d(Point::new(x, y), pos.1);
                    }
                    DistanceAlgorithm::Chebyshev =>
                    {
                        distance = DistanceAlg::Chebyshev.distance2d(Point::new(x, y), pos.1);
                    }
                }
                voronoi_distance[seed] = (seed,distance);
            }
            voronoi_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
            
            *vid = voronoi_distance[0].0 as i32;
        }

        for y in 1..MAPHEIGHT-1
        {
            for x in 1..MAPWIDTH-1
            {
                let mut neighbors = 0;
                let my_idx = Map::xy_id(x, y);
                let my_seed = voronoi_membership[my_idx];

                if voronoi_membership[Map::xy_id(x-1, y)] != my_seed {neighbors += 1;}
                if voronoi_membership[Map::xy_id(x+1, y)] != my_seed {neighbors += 1;}
                if voronoi_membership[Map::xy_id(x, y-1)] != my_seed {neighbors += 1;}
                if voronoi_membership[Map::xy_id(x, y+1)] != my_seed {neighbors += 1;}

                if neighbors < 2
                {
                    build_data.map.map[my_idx] = TileType::Floor;
                }
            }
        }

    }
}