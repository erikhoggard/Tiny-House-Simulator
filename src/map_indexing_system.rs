use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();

        // join to get a list of all entities with a position
        for (entity, position) in (&entities, &position).join() {
            // for this entity, get its index
            let idx = map.xy_idx(position.x, position.y);

            // okay so:
            // we check the list of blockers and get its entity.
            // because this might be null, we have to check Some()
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            //push the entity to the index slot.
            // entity is a copy typem so cloning is not necessary.
            map.tile_content[idx].push(entity);
        }
    }
}
