use bevy::{DefaultPlugins, window::{Windows, WindowDescriptor}, math::vec3};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::movement::*;
use crate::collision::*;
use crate::demos::GravityDemos::*;

mod movement;
mod collision;
mod demos;

pub struct TimeScale{
    scale: f32,
}

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
        .insert_resource(TimeScale{ scale: 1. })
        .insert_resource(GravitationalConstant{
            //due to floating point rounding errors don't use the actual gravitational constant
            //g: 6.6743 * 1. / (10. as f64).powf(11.) as f64,
            g: 5000.,
        })
        .insert_resource(WindowDescriptor{
			title: "Physics Engine".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
			..Default::default()
		})
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        //.add_startup_system(four_body)
        .add_system(gravity_sys)
        .add_system(movement_sys)
        .add_system(collision_sys)
        .run();
}

fn setup_system(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
) {
	// camera    
    let mut cam = Camera2dBundle::default();    
    cam.projection.scale = 2.;
	commands.spawn_bundle(cam);	

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
    let circle = create_reg_poly(100, 50.);

    commands.spawn_bundle(GeometryBuilder::build_as(
        &circle,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform{
            translation: vec3( -200., 0.,0.),
            ..default()
        },
    )).insert(RigidBody {
        mass: Some(200.),
        vel: vec3(10., 0., 0.),
        ..default()
    }).insert(CircleCollision{
        radius: 50.
    });
    
    commands.spawn_bundle(GeometryBuilder::build_as(
        &circle,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform{
            translation: vec3( 200., 0.,0.),
            ..default()
        },
    )).insert(RigidBody {
        mass: Some(200.),
        vel: vec3(-10., 0., 0.),
        ..default()
    }).insert(CircleCollision{
        radius: 50.
    });
}


fn create_square_sprite(
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

fn create_reg_poly(
    sides: usize,
    radius: f32,
) -> RegularPolygon {
    return shapes::RegularPolygon {
        sides: sides,
        feature: RegularPolygonFeature::Radius(radius),
        ..shapes::RegularPolygon::default()
    };
}