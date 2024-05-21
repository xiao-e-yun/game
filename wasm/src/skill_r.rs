use std::borrow::Borrow;

use butterfly_soul_engine::{
  defined_event,
  indexmap::IndexMap,
  modules::{
    context::render::Texture,
    enity::{position::MoveEvent, track::EnityTrack, view::EnityView, Enity},
    scene::NormalScene,
  },
  utils::{bse_map::BseMap, rchash::RcHash, rect::Rect, vector::Vector, viewbox::ViewBox},
};

use crate::{
  bullet::{set_attackable, Bullet, BulletAttackEventArg, BulletNextEventArg},
  enemy::Enemy,
  role::Role,
};

impl Role {
  pub fn skill_r(
    &self,
    scene: &NormalScene,
    mouse: Vector,
    enemies: BseMap<Enemy>,
  ) -> Option<Bullet> {
    let mut r_status = self.skill_r.borrow_mut();
    if !r_status.ready() {
      return None;
    }

    r_status.reset_cd();
    r_status.using = true;

    let scene_uuid = scene.uuid();
    let from = self.position(scene_uuid).get();
    let offset = from.to(mouse);
    let distance = offset.distance();

    let offset_to = if distance > 300.0 {
      offset.by_length(distance - 300.)
    } else {
      Vector::ORIGIN
    };
    self.position(scene_uuid).set_action(MoveEvent::Drift(
      offset_to,
      0.000_2 * (distance - 300.).max(0.),
    ));

    let viewbox = (
      Rect::new(Vector::ORIGIN, Vector::new(800., 500.)),
      Texture::Color("#2e2e364d".to_string()),
    );

    let mut bullet = Bullet::new("skill.r", "enemy", viewbox, 0.);
    bullet.add(SkillRAttack::new(enemies));
    bullet.add(SkillRNext::new(
      bullet.track().clone(),
      self.skill_r.clone(),
    ));

    {
      let mut position = bullet.position(scene_uuid);
      position.set(mouse);
    };

    Some(bullet)
  }
}

#[derive(Debug, Clone)]
pub struct SkillRStatus {
  pub using: bool,
  cd: usize,
}

impl SkillRStatus {
  pub fn new() -> Self {
    Self {
      cd: 0,
      using: false,
    }
  }
  pub fn reset_cd(&mut self) {
    self.cd = 5000;
  }
  pub fn cd(&mut self, delta: usize) {
    self.cd = self.cd.saturating_sub(delta);
  }
  pub fn ready(&self) -> bool {
    self.cd == 0
  }
}

#[derive(Debug, Clone)]
pub struct SkillRAttack {
  targets: BseMap<Enemy>,
}

impl SkillRAttack {
  pub fn new(targets: BseMap<Enemy>) -> Self {
    Self { targets }
  }
}

defined_event!(SkillRAttack: BulletAttackEventArg,|this: &mut SkillRAttack,uuid|{
  let target = this.targets.get(uuid).unwrap();
  let mut target = target.borrow_mut();
  target.base_mut().destroy();

  false
});

#[derive(Debug, Clone)]
pub struct SkillRNext {
  status: RcHash<SkillRStatus>,
  bullet: EnityTrack,
  lifttime: usize,
}

impl SkillRNext {
  pub fn new(bullet: EnityTrack, status: RcHash<SkillRStatus>) -> Self {
    let lifttime = 1000;
    Self {
      bullet,
      lifttime,
      status,
    }
  }
}

defined_event!(SkillRNext: BulletNextEventArg,|this: &mut SkillRNext,delta: &usize|{
  let bullet = &this.bullet;
  let before = this.lifttime;
  this.lifttime = before.saturating_sub(*delta);

  let tick = this.lifttime % 400;
  if tick + delta >= 300 && tick <= 300 {

    bullet.view_mut().insert("base".to_string(), vec![(
      Rect::new(Vector::ORIGIN, Vector::new(750., 450.)),
      Texture::Color("#ff505011".to_string()),
    )]);

  }

  if tick + delta >= 400 {
    set_attackable(bullet,true);

    bullet.view_mut().insert("base".to_string(), vec![(
      Rect::new(Vector::ORIGIN, Vector::new(750., 450.)),
      Texture::Color("#ff505033".to_string()),
    )]);

  }

  if this.lifttime == 0 {
    this.status.borrow_mut().using = false;
    bullet.base_mut().destroy();
  }
  false
});
