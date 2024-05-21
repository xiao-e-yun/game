use std::borrow::{Borrow, BorrowMut};

use butterfly_soul_engine::{
  create_event_arg,
  modules::{
    context::render::Texture,
    enity::{base::EnityBase, track::EnityTrack, view::EnityView, Enity},
    scene::NormalScene,
  },
  utils::{
    bse_map::BseMapNode, event::{Event, Events}, rchash::RcHash, rect::Rect, vector::Vector
  },
  uuid::Uuid,
};

//
//
//
pub fn set_attackable(bullet: &EnityTrack,active: bool) {
  let mut base = bullet.base_mut();
  if active { base.add_group("attackable"); }
  else { base.remove_group("attackable"); };
}

pub fn attackable(bullet: &EnityTrack) -> bool {
  bullet.base_mut().has_group("attackable")
}

//
//
//

#[derive(Debug)]
pub struct Bullet {
  target: String,
  track: EnityTrack,
  events: Events,
}

impl BseMapNode for Bullet {
  type Key = Uuid;
  fn get_key(&self) -> Self::Key {
    self.uuid()
  }
}

impl Bullet {
  pub fn new(name: &str, target: &str, viewbox: (Rect, Texture), speed: f32) -> Self {
    let mut base = EnityBase::new(name.to_string(), vec!["bullet".to_string()], speed);
    let view = EnityView::new(vec![viewbox.clone()], vec![viewbox.0]);
    base.set_no_collision();

    Bullet {
      target: target.to_string(),
      track: EnityTrack::new(base, view),
      events: Events::new(),
    }
  }

  pub fn add<T: Event + 'static>(&mut self, event: T) {
    self.events.add(event)
  }

  pub fn trigger(&mut self, scene: &NormalScene,delta: usize) {

    self.events.trigger::<BulletNextEventArg>(delta);

    if attackable(self.track()) {
      set_attackable(self.track(), false);

      for enity in scene.collision(self.track()).iter() {
        if !enity.base().has_group(&self.target) {
          continue;
        }
        self.events.trigger::<BulletAttackEventArg>(enity.uuid())
      }
    }
  }
}

impl Enity for Bullet {
  fn track(&self) -> &EnityTrack {
    &self.track
  }
}


create_event_arg!("bullet.attack", BulletAttackEventArg: Uuid);
create_event_arg!("bullet.next", BulletNextEventArg: usize);
