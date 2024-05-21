use std::borrow::Borrow;

use butterfly_soul_engine::{
  defined_event,
  indexmap::IndexMap,
  modules::{
    context::render::Texture,
    enity::{base::EnityBase, position::MoveEvent, track::EnityTrack, view::EnityView, Enity},
    scene::NormalScene,
  },
  utils::{bse_map::BseMap, rchash::RcHash, rect::Rect, vector::Vector},
  uuid::Uuid,
};

use crate::{
  bullet::{set_attackable, Bullet, BulletAttackEventArg, BulletNextEventArg},
  enemy::Enemy,
  role::Role,
};

impl Role {
  pub fn skill_e(
    &self,
    scene: &NormalScene,
    mouse: Vector,
    enemies: BseMap<Enemy>,
  ) -> Option<Bullet> {
    let mut e_status = self.skill_e.borrow_mut();
    if !e_status.ready() {
      return None;
    }
    let targets = scene.collision_by_point(mouse);
    let mut targets = targets.iter().filter(|v|v.base().has_group("enemy"));
    let target = targets.next()?;
    let scene_uuid = scene.uuid();
    let from = self.position(scene_uuid).get();
    let offset = from.to(target.position(scene_uuid).get());
    let offset_to = offset.by_length(200.);
    
    if offset.distance() > 200. {
      return None;
    }

    e_status.reset_cd();
    self.position(scene_uuid).offset(offset_to, 0.16);
    
    let viewbox = (
      Rect::new(Vector::ORIGIN, Vector::new(200., 50.)),
      Texture::Color("#2e2e364d".to_string()),
    );

    let mut bullet = Bullet::new("skill.e", "enemy", viewbox, 0.);
    // bullet.view_mut().remove("base");
    bullet.add(SkillEAttack::new(self.skill_e.clone(), enemies));
    bullet.add(SkillENext::new(bullet.track().clone()));

    {
      let mut position = bullet.position(scene_uuid);
      position.set(from + offset_to * 0.5);
      position.set_angle(offset.radian());
    };

    Some(bullet)
  }
}

#[derive(Debug, Clone)]
pub struct SkillEStatus {
  cd: usize,
}

impl SkillEStatus {
  pub fn new() -> Self {
    Self { cd: 0 }
  }
  pub fn reset_cd(&mut self) {
    self.cd = 200;
  }
  pub fn cd(&mut self, delta: usize) {
    self.cd = self.cd.saturating_sub(delta);
  }
  pub fn ready(&self) -> bool {
    self.cd == 0
  }
}

#[derive(Debug,Clone)]
pub struct SkillEAttack {
  targets: BseMap<Enemy>,
  _status: RcHash<SkillEStatus>,
}

impl SkillEAttack {
  pub fn new(status: RcHash<SkillEStatus>, targets: BseMap<Enemy>) -> Self {
    Self {
      targets,
      _status: status,
    }
  }
}

defined_event!(SkillEAttack: BulletAttackEventArg,|this: &mut SkillEAttack,uuid|{
  let target = this.targets.get(uuid).unwrap();
  let mut target = target.borrow_mut();
  target.base_mut().destroy();

  false
});

#[derive(Debug,Clone)]
pub struct SkillENext {
  bullet: EnityTrack,
  lifttime: usize,
}

impl SkillENext {
  pub fn new(bullet: EnityTrack) -> Self {
    let lifttime = 160;
    Self {
      bullet,
      lifttime,
    }
  }
}

defined_event!(SkillENext: BulletNextEventArg,|this: &mut SkillENext,delta: &usize|{
  let bullet = &this.bullet;
  let before = this.lifttime;
  this.lifttime = before.saturating_sub(*delta);

  if this.lifttime == 0 {
    set_attackable(bullet,true);
    bullet.base_mut().destroy();
  }
  false
});
