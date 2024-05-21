use std::{borrow::Borrow, f32::consts::PI};

use bse_web::Context;
use butterfly_soul_engine::{
  modules::{
    context::{
      control::{Control, GetMoveVector},
      render::{Render, RenderFrame},
    },
    enity::{position::MoveEvent, track::EnityTrack, Enity},
    scene::NormalScene,
  },
  utils::{bse_map::BseMap, rchash::RcHash, vector::Vector},
};
use enemy::Enemy;
use rand::random;
use role::Role;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::bullet::Bullet;
pub use bse_web::WebContext;

mod bullet;
mod enemy;
mod role;
mod skill_e;
mod skill_q;
mod skill_r;
mod utils;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! check_key {
    ($check: expr,$key:ident$(.$ext: ident)* ) => {
      &$check.code == stringify!($key) $(&& $check.$ext)*
    };
}

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once()
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GameCore {
  role: RcHash<Role>,
  enemies: BseMap<Enemy>,
  bullets: BseMap<Bullet>,
  scense: NormalScene,
  control: Control,
  paused: bool,
  time: usize,
}

#[wasm_bindgen]
impl GameCore {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut scense = NormalScene::new(Vector::new(500000., 500000.));
    // scense.set_background(Texture::Color("#000".to_string()));

    let role = Role::new();
    let role_track = role.borrow().track().clone();
    scense.insert(&role_track);

    let enemies = BseMap::new();
    for i in 0..9 {
      if i == 4 {
        continue;
      };
      let enemy = Enemy::new();
      let track = enemy.track().clone();
      let mut position = track.position(scense.uuid());
      position.set_action(MoveEvent::Following(role_track.clone()));
      position.set(Vector((i / 3 - 1) as f32 * 300., (i % 3 - 1) as f32 * 300.));

      scense.insert(&track);
      enemies.insert(RcHash::new(enemy));
    }

    GameCore {
      control: Control::new(),
      role,
      bullets: BseMap::new(),
      paused: false,
      time: 0,
      enemies,
      scense,
    }
  }
  pub fn update(&mut self, ctx: &WebContext, delta: usize) -> Result<(), String> {
    self.control(ctx)?;
    self.main(delta)?;
    Ok(())
  }

  pub fn render(&self, ctx: &WebContext) -> Result<(), String> {
    let mut frame = RenderFrame::new();
    self.scense.render(&mut frame);
    ctx.render(frame.clone());
    Ok(())
  }
}

impl GameCore {
  pub fn control(&mut self, ctx: &WebContext) -> Result<(), String> {
    self.control = ctx.control().unwrap();
    Ok(())
  }

  pub fn main(&mut self, delta: usize) -> Result<(), String> {
    let toggle_pause = self
      .control
      .keys()
      .iter()
      .find(|key| check_key!(key, Escape) && !key.repeat)
      .is_some();
    if toggle_pause {
      self.paused = !self.paused;
      // if self.paused {
      //   self.scense.remove_ui()
      // } else {

      // }
    };

    // not paused
    if !self.paused {
      //===================================================================
      // Role
      //===================================================================
      {
        let mut role = self.role.borrow_mut();
        let control = &self.control;
        let mouse = self.scense.viewport().map_to_viewport(control.mouse);

        {
          let mut position = role.position(self.scense.uuid());
          if !role.skill_r.borrow().using {
            position.set_action(MoveEvent::Moving(control.move_vector()))
          };

          self.scense.viewport_mut().set_position(position.get())
        };

        role.skill_cd(delta);

        let keys = control.keys();
        for key in keys {
          if check_key!(key, KeyQ) {
            let bullet = role.skill_q(self.scense.uuid(), mouse, self.enemies.clone());
            if let Some(bullet) = bullet {
              self.scense.insert(bullet.track());
              self.bullets.insert(RcHash::new(bullet));
            }
          }
          if check_key!(key, KeyE) {
            let bullet = role.skill_e(&self.scense, mouse, self.enemies.clone());
            if let Some(bullet) = bullet {
              self.scense.insert(bullet.track());
              self.bullets.insert(RcHash::new(bullet));
            }
          }
          if check_key!(key, KeyR) {
            let bullet = role.skill_r(&self.scense, mouse, self.enemies.clone());
            if let Some(bullet) = bullet {
              self.scense.insert(bullet.track());
              self.bullets.insert(RcHash::new(bullet));
            }
          }
        }
      };

      //===================================================================
      // Enemy
      //===================================================================
      if self.time % 1000 + delta >= 1000 {
        let role = self.role.borrow();
        let role_track = role.track();
        let role_pos = role.position(self.scense.uuid());
        let position = role_pos.get() + Vector::new(1600., 0.).rotate(2.0 * PI * random::<f32>());

        for i in 0..25 {
          let enemy = Enemy::new();
          let track = enemy.track().clone();

          let rel_pos = Vector::new((i / 5 - 2) as f32 * 50., (i % 5 - 2) as f32 * 50.);
          let spawn_position = position + rel_pos;

          let mut position = track.position(self.scense.uuid());
          position.set_action(MoveEvent::Following(role_track.clone()));
          position.set(spawn_position);

          self.scense.insert(&track);
          self.enemies.insert(RcHash::new(enemy));
        }
      }

      //===================================================================
      // Bullet
      //===================================================================
      for bullet in self.bullets.clone().values() {
        let mut bullet = bullet.borrow_mut();
        bullet.trigger(&self.scense, delta);

        if bullet.base().is_destroy() {
          self.bullets.swap_remove(&bullet.uuid());
        }
      }

      self.scense.update(delta);
      self.time += delta;
    }

    Ok(())
  }
}
