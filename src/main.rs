use bevy::{DefaultPlugins, core::Time, window::{Windows, WindowDescriptor}, ecs::bundle, render::texture::DEFAULT_IMAGE_HANDLE, math::{vec2, vec3}};
use bevy::prelude::*;
mod components;
mod systems;

use components::{acceleration::Acc2, system_identity_components::{Movable, ExperienceDrag}, drag::Drag2, mass::{self, Mass}};
use systems::movable_system::move_entities;

use crate::systems::drag_system::apply_drag;

pub struct TimeStep{
    time: Time,
    time_step: f32,
}

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

fn main() {
    println!("Hello, world!");

    App::new()
        .insert_resource(TimeStep{
            time: Default::default(),
            time_step: 1./64.
        })
        .insert_resource(WindowDescriptor {
			title: "Physics Engine".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
			..Default::default()
		})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        .add_system(move_entities)
        .add_system(apply_drag)
        .run();
}

fn setup_system(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
) {
	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	// position window (for tutorial)
	// window.set_position(IVec2::new(2780, 4900));

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
}

fn spawn_entities(
    mut commands: Commands,
){
    // square
    commands.spawn_bundle((       
        Transform {
            translation: vec3(0., 0., 0.),
            scale: vec3(100., 100., 100.),
            rotation: Quat::IDENTITY,
        },
        Sprite {
            color: Color::rgb(1., 1., 1.),
            ..Default::default()
        },
        GlobalTransform::default(),
        Visibility::default(),
        DEFAULT_IMAGE_HANDLE.typed::<Image>(),
        Acc2{
            acc: vec2(0., 0.),
            vel: vec2(1., 0.),
        },
        Drag2{
            drag_coefficient: 1000.5,
            fluid_density: 1.2754,
            fluid_vel: vec2(0., 0.),
        },
        Mass{
            mass: 100.,
        },
        Movable,
        ExperienceDrag,
    ));
}
