use engine::{
  application::scene::{Collision, Scene, TransformComponent},
  nalgebra::Vector3,
  systems::{Backpack, Initializable, Inventory, System},
};
use rand::Rng;

use super::components::{Ball, Pickup, SpawnArea, Flapper};

pub struct FlapperSystem {
  test : f32 
}

impl Initializable for FlapperSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {
      test : 0.0, 
    }
  }
}

impl System for FlapperSystem {
  fn get_name(&self) -> &'static str {
    "FlapperSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_flapper_motion(scene, backpack);
  }
}

impl FlapperSystem {
  fn handle_flapper_motion(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    for (entity, (transform, flapper)) in scene.query_mut::<(&mut TransformComponent, &Flapper)>(){
      transform.translation.x = self.test.sin();
    };
    self.test += 1.0;
   Some(())
  }
}
