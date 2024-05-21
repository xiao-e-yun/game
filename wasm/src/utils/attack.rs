#[derive(Debug, Clone)]
pub struct AttackStatus {
  pub target: Group,
  pub lifetime: f32,
  pub hittime: f32,
}

impl AttackStatus {
  pub fn new(
    target: Group,
    lifetime: f32,
    hittime: f32,
  ) -> AttackStatus {
    AttackStatus {
      lifetime,
      hittime,
      target,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Group {
  Friendly,
  Hostility,
}