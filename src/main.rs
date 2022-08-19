use bevy::{DefaultPlugins, window::{Windows, WindowDescriptor}, render::texture::DEFAULT_IMAGE_HANDLE, math::vec3, reflect::erased_serde::private::serde::__private::de};
use bevy::prelude::*;
use bevy::time::*;

use crate::movement::*;
use crate::collision::*;

mod movement;
mod collision;

pub struct GravitationalConstant{
    g: f32
}

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(Time::default())
        .insert_resource(GravitationalConstant{
            //g: 6.6743 * 1. / (10. as f64).powf(11.) as f64,
            g: 500.,
        })
        .insert_resource(WindowDescriptor {
			title: "Physics Engine".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
			..Default::default()
		})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        .add_system(gravity_sys)
        .add_system(movement_sys)
        .run();
}

fn setup_system(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
) {
	// camera    
    // let mut cam = Camera2dBundle::default();
    // cam.projection.scale = 2.;
	// commands.spawn_bundle(cam);
	commands.spawn_bundle(Camera2dBundle::default());

	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
}

fn spawn_entities(
    mut commands: Commands,
){    
    commands.spawn_bundle(create_sprite(
        Color::rgb(1., 1., 1.), 
        vec3( 200., 0.,0.), 
        vec3(100., 100., 100.)
    ))
    .insert(RigidBody {
        mass: Some(200.),        
        vel: vec3(0., -100., 0.),
        ..default()
    });

    commands.spawn_bundle(create_sprite(
        Color::rgb(1., 1., 1.), 
        vec3( -200., 0.,0.), 
        vec3(100., 100., 100.)
    ))
    .insert(RigidBody {
        mass: Some(200.),        
        vel: vec3(0., 100., 0.),
        ..default()
    });

    commands.spawn_bundle(create_sprite(
        Color::rgb(1., 1., 1.), 
        vec3( 0., 200.,0.), 
        vec3(100., 100., 100.)
    ))
    .insert(RigidBody {
        mass: Some(200.),        
        vel: vec3(100., 0., 0.),
        ..default()
    });

    commands.spawn_bundle(create_sprite(
        Color::rgb(1., 1., 1.), 
        vec3( 0., -200.,0.), 
        vec3(100., 100., 100.)
    ))
    .insert(RigidBody {
        mass: Some(200.),        
        vel: vec3(-100., 0., 0.),
        ..default()
    });
}

fn create_sprite(
    colour: Color,
    translation: Vec3,
    scale: Vec3,
) -> SpriteBundle {
    return SpriteBundle{
        sprite: Sprite { 
            color: colour,
            ..default()
        },
        transform : Transform {
            translation: translation,
            scale: scale,
            rotation: Quat::IDENTITY,
        },
        ..default()
    }
}
