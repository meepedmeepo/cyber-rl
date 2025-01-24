use super::MetaMapBuilder;




pub struct DoglegCorridors {}

impl MetaMapBuilder for DoglegCorridors
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) 
    {
        
    }
}

impl DoglegCorridors
{
    fn new() -> Box<DoglegCorridors>
    {
        Box::new(DoglegCorridors{})
    }

    fn build()
    {
        
    }
}
