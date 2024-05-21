use std::cell::Ref;

use butterfly_soul_engine::{
  modules::{
    context::render::Texture,
    enity::{base::EnityBase, track::EnityTrack, view::EnityView, Enity},
  },
  utils::{
    bse_map::BseMap,
    rchash::{RcHash, WeakHash},
    rect::Rect,
    vector::Vector,
  },
  uuid::Uuid,
};

use crate::{skill_e::SkillEStatus, skill_q::SkillQStatus, skill_r::SkillRStatus};

#[derive(Debug, Clone)]
pub struct Role {
  weak: WeakHash<Role>,
  track: EnityTrack,
  pub skill_e: RcHash<SkillEStatus>,
  pub skill_q: RcHash<SkillQStatus>,
  pub skill_r: RcHash<SkillRStatus>,
}

impl Role {
  pub fn new() -> RcHash<Self> {
    let size = Vector::new(50., 50.);
    let rect = Rect::new(Vector::ORIGIN, size);
    let texture = Texture::Color("#777".to_string());

    let base = EnityBase::new("role".to_string(), vec!["role".to_string()], 100.);
    let view = EnityView::new(vec![(rect, texture)], vec![rect]);
    let track = EnityTrack::new(base, view);

    let skill_q = SkillQStatus::new();
    let skill_e = SkillEStatus::new();
    let skill_r = SkillRStatus::new();

    let rc = RcHash::new(Self {
      track,
      skill_q: RcHash::new(skill_q),
      skill_r: RcHash::new(skill_r),
      skill_e: RcHash::new(skill_e),
      weak: WeakHash::null(),
    });

    rc.borrow_mut().weak = rc.downgrade();
    rc
  }
  pub fn self_ref(&self) -> RcHash<Self> {
    self.weak.clone().upgrade().unwrap()
  }

  pub fn skill_cd(&mut self, delta: usize) {
    self.skill_q.borrow_mut().cd(delta);
    self.skill_e.borrow_mut().cd(delta);
    self.skill_r.borrow_mut().cd(delta);
  }
}

impl Enity for Role {
  fn track(&self) -> &EnityTrack {
    &self.track
  }
}
