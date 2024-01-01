//! Genuary 2024 Day 1
//! The goal was learning, but after an hour of tinkering, I didn't learn much.
//! I really just tinkered with this source:
//! https://github.com/djeedai/bevy_hanabi/blob/main/examples/expr.rs
//! 
//! That's what dailie are for :) Let's shoor for something interesting by end of month.
//! 

use bevy::{
  core_pipeline::{
      bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
  },
  log::LogPlugin,
  prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_hanabi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  App::default()
      .add_plugins(
          DefaultPlugins
              .set(LogPlugin {
                  level: bevy::log::Level::WARN,
                  filter: "bevy_hanabi=warn,expr=trace".to_string(),
              })
              .set(WindowPlugin {
                  primary_window: Some(Window {
                      title: "🎆 Hanabi — expr".to_string(),
                      ..default()
                  }),
                  ..default()
              }),
      )
      .add_systems(Update, bevy::window::close_on_esc)
      .add_plugins(HanabiPlugin)
      .add_plugins(WorldInspectorPlugin::default())
      .add_systems(Startup, setup)
      .run();

  Ok(())
}


fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
  commands.spawn((
      Camera3dBundle {
          transform: Transform::from_translation(Vec3::new(3., 12., 20.))
              .looking_at(Vec3::Y * 5., Vec3::Y),
          camera: Camera {
              hdr: true,
              ..default()
          },
          camera_3d: Camera3d {
              clear_color: ClearColorConfig::Custom(Color::BLACK),
              ..default()
          },
          tonemapping: Tonemapping::AgX,
          ..default()
      },
      BloomSettings::default(),
  ));


  // Set gradient over the lifetime of the particle
  let mut color_gradient = Gradient::new();
  for (t, (r, g, b, a)) in [
    (0.0, (0.0, 0.0, 0.0, 0.25)),
    (0.0, (0.5, 0.5, 0.5, 0.75)),
    (1.0, (4.0, 4.0, 4.0, 1.0)),
  ] {
    color_gradient.add_key(t, Vec4::new(r, g, b, a))
  }

  let mut size_gradient = Gradient::new();
  for (t, (w, l)) in [
    (0.5, (0.5, 0.125)),
    (1.0, (0.0625, 0.016125))
  ] {
    size_gradient.add_key(t, Vec2::new(w, l));
  }

  let writer = ExprWriter::new();

  let age = writer.lit(0.).expr();
  let init_age = SetAttributeModifier::new(Attribute::AGE, age);

  // Give a bit of variation by randomizing the lifetime per particle
  let lifetime = writer.lit(2.5).uniform(writer.lit(3.5)).expr();
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  // Create some whirlwind effect by adding some radial acceleration pointing at
  // the origin (0,0,0) and some upward acceleration (alongside Y). The proportion
  // of radial acceleration varies based on the simulation time.
  let pos = writer.attr(Attribute::POSITION);
  let zero = writer.lit(Vec3::ZERO);
  let radial = (pos - zero).normalized();
  let vertical = writer.lit(Vec3::Y * 4.);
  let anim = writer.time().sin() * writer.lit(6.) - writer.lit(6.);
  let accel = anim;
  //let accel = radial * anim + vertical;

  let update_accel = AccelModifier::new(accel.expr());

  let init_pos = SetPositionCircleModifier {
      center: writer.lit(Vec3::ZERO).expr(),
      axis: writer.lit(Vec3::Y).expr(),
      radius: writer.lit(4.).expr(),
      dimension: ShapeDimension::Surface,
  };

  let init_vel = SetVelocityTangentModifier {
      origin: writer.lit(Vec3::ZERO).expr(),
      axis: writer.lit(Vec3::Y).expr(),
      speed: writer.lit(3.).expr(),
  };

  let effect = effects.add(
      EffectAsset::new(32768, Spawner::rate(1000.0.into()), writer.finish())
          .with_name("whirlwind")
          .init(init_pos)
          .init(init_age)
          .init(init_lifetime)
          .init(init_vel)
          .update(update_accel)
          .render(ColorOverLifetimeModifier {
              gradient: color_gradient,
          })
          .render(SizeOverLifetimeModifier {
              gradient: size_gradient,
              screen_space_size: false,
          })
          .render(OrientModifier::new(OrientMode::AlongVelocity)),
  );

  commands.spawn((
      Name::new("whirlwind"),
      ParticleEffectBundle {
          effect: ParticleEffect::new(effect),
          transform: Transform::IDENTITY,
          ..Default::default()
      },
  ));
}