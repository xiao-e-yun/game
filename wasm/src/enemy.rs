use butterfly_soul_engine::{
  modules::{
    context::render::Texture,
    enity::{base::EnityBase, track::EnityTrack, view::EnityView, Enity},
  },
  utils::{bse_map::BseMapNode, rect::Rect, vector::Vector}, uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct Enemy {
  track: EnityTrack,
}

impl BseMapNode for Enemy {
    type Key = Uuid;
    fn get_key(&self) -> Self::Key {
      self.uuid()
    }
}

impl Enemy {
  pub fn new() -> Self {
    let position = Vector::ORIGIN;
    let size = Vector::new(30., 30.);
    let rect = Rect::new(position, size);
    let texture = Texture::Color("#bbb".to_string());

    let base = EnityBase::new("enemy".to_string(), vec!["enemy".to_string()], 30.);
    let view = EnityView::new(vec![(rect, texture)], vec![rect]);

    Self {
      track: EnityTrack::new(base, view),
    }
  }
}

impl Enity for Enemy {
  fn track(&self) -> &EnityTrack {
    &self.track
  }
}
