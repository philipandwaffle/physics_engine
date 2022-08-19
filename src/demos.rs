use bevy_prototype_lyon::prelude::*;

pub mod GravityDemos{
    use bevy::{prelude::*, math::vec3};
    use bevy_prototype_lyon::prelude::*;
    use crate::{collision::CircleCollision, movement::RigidBody};

    use super::create_reg_poly;

    pub fn four_body(
        mut commands: Commands,
    ){
        let circle = create_reg_poly(100, 50.);

        commands.spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform{
                translation: vec3( 0., 200.,0.),
                ..default()
            },
        )).insert(RigidBody {
            mass: Some(200.),
            vel: vec3(-100., 0., 0.),
            ..default()
        }).insert(CircleCollision{
            radius: 50.
        });
        
        commands.spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform{
                translation: vec3( 0., -200.,0.),
                ..default()
            },
        )).insert(RigidBody {
            mass: Some(200.),
            vel: vec3(100., 0., 0.),
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
            vel: vec3(0., 100., 0.),
            ..default()
        }).insert(CircleCollision{
            radius: 50.
        });

        commands.spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform{
                translation: vec3( -200., 0.,0.),
                ..default()
            },
        )).insert(RigidBody {
            mass: Some(200.),
            vel: vec3(0., -100., 0.),
            ..default()
        }).insert(CircleCollision{
            radius: 50.
        });
    }

    pub fn two_body(
        mut commands: Commands,
    ){
        let circle = create_reg_poly(100, 50.);

        commands.spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform{
                translation: vec3( 0., 200.,0.),
                ..default()
            },
        )).insert(RigidBody {
            mass: Some(200.),
            vel: vec3(-100., 0., 0.),
            ..default()
        }).insert(CircleCollision{
            radius: 50.
        });
        
        commands.spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform{
                translation: vec3( 0., -200.,0.),
                ..default()
            },
        )).insert(RigidBody {
            mass: Some(200.),
            vel: vec3(100., 0., 0.),
            ..default()
        }).insert(CircleCollision{
            radius: 50.
        });
    }
}

pub fn create_reg_poly(
    sides: usize,
    radius: f32,
) -> RegularPolygon {
    return shapes::RegularPolygon {
        sides: sides,
        feature: RegularPolygonFeature::Radius(radius),
        ..shapes::RegularPolygon::default()
    };
}