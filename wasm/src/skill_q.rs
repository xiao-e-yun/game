use butterfly_soul_engine::{
  defined_event,
  indexmap::IndexMap,
  modules::{
    context::render::Texture,
    enity::{base::EnityBase, position::MoveEvent, track::EnityTrack, view::EnityView, Enity},
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
  pub fn skill_q(&self, scene_uuid: Uuid, mouse: Vector, enemies: BseMap<Enemy>) -> Option<Bullet> {
    let mut q_status = self.skill_q.borrow_mut();
    if !q_status.ready() {
      return None;
    }
    q_status.reset_cd();
    let advanced = q_status.try_advanced();

    let viewbox = if advanced {
      (
        Rect::new(Vector::ORIGIN, Vector::new(120., 120.)),
        Texture::Color("#2e2e364d".to_string()),
      )
    } else {
      (
        Rect::new(Vector::ORIGIN, Vector::new(120., 30.)),
        Texture::Color("#2e2e364d".to_string()),
      )
    };


    let mut bullet = Bullet::new("skill.q", "enemy", viewbox, 400.);
    bullet.add(SkillQAttack::new(self.skill_q.clone(), enemies, advanced));
    bullet.add(SkillQNext::new(bullet.track().clone(), advanced));

    {
      let mut position = bullet.position(scene_uuid);
      let from = self.position(scene_uuid).get();
      let offset = from.to(mouse);
      position.set(from + offset.by_length(100.));
      position.set_angle(offset.radian());

      if advanced {
        position.set_action(MoveEvent::Moving(offset))
      }
    };

    Some(bullet)
  }
}

#[derive(Debug, Clone)]
pub struct SkillQStatus {
  cd: usize,
  stacks: usize,
  stacks_time: usize,
}

impl SkillQStatus {
  pub fn new() -> Self {
    Self {
      cd: 0,
      stacks: 0,
      stacks_time: 0,
    }
  }
  pub fn reset_cd(&mut self) {
    self.cd = 500;
  }
  pub fn cd(&mut self, delta: usize) {
    self.cd = self.cd.saturating_sub(delta);
    self.stacks_time = self.stacks_time.saturating_sub(delta);
  }
  pub fn get_stack(&self) -> usize {
    if self.stacks_time == 0 {
      0
    } else {
      self.stacks
    }
  }
  pub fn try_advanced(&mut self) -> bool {
    let advanced = self.get_stack() == 2;
    if advanced {
      self.reset_stack()
    }
    advanced
  }
  pub fn update_stack(&mut self) {
    self.stacks = self.get_stack() + 1;
    self.stacks_time = 4000;
  }
  pub fn reset_stack(&mut self) {
    self.stacks = 0;
    self.stacks_time = 0;
  }
  pub fn ready(&self) -> bool {
    self.cd == 0
  }
}

#[derive(Debug,Clone)]
pub struct SkillQAttack {
  targets: BseMap<Enemy>,
  status: RcHash<SkillQStatus>,
  advanced: bool,
  attacked: bool,
}

impl SkillQAttack {
  pub fn new(status: RcHash<SkillQStatus>, targets: BseMap<Enemy>, advanced: bool) -> Self {
    Self {
      targets,
      status,
      advanced,
      attacked: false,
    }
  }
}

defined_event!(SkillQAttack: BulletAttackEventArg,|this: &mut SkillQAttack,uuid|{
  let target = this.targets.get(uuid).unwrap();
  let mut target = target.borrow_mut();
  target.base_mut().destroy();

  if this.advanced {

    //todo

  } else {

    if !this.attacked {
      this.status.borrow_mut().update_stack();
      this.attacked = true;
    }

  }

  false
});

#[derive(Debug,Clone)]
pub struct SkillQNext {
  bullet: EnityTrack,
  lifttime: usize,
  advanced: bool,
}

impl SkillQNext {
  pub fn new(bullet: EnityTrack, advanced: bool) -> Self {
    let lifttime = if advanced { 1000 } else { 40 };
    Self {
      bullet,
      advanced,
      lifttime,
    }
  }
}

defined_event!(SkillQNext: BulletNextEventArg,|this: &mut SkillQNext,delta: &usize|{
  let bullet = &this.bullet;
  let before = this.lifttime;
  this.lifttime = before.saturating_sub(*delta);

  if this.advanced {

    let tick_time = 200;
    let (before_tick,now_tick) = (before / tick_time,this.lifttime / tick_time);
    let attackable = before_tick - now_tick != 0;
    if attackable {
      let mut view = bullet.view_mut();
      let (mut viewbox, texture) = view.remove("base").unwrap().pop().unwrap();
      viewbox.size += 2. * *delta as f32;
      view.insert("base".to_string(),vec![(viewbox,texture)]);
      view.insert_hitbox("base".to_string(),vec![viewbox]);
      set_attackable(bullet,true);
    }

  } else {

    if this.lifttime == 0 {
      set_attackable(bullet,true);
    };

  }

  if this.lifttime == 0 {
    bullet.base_mut().destroy();
  }
  false
});
