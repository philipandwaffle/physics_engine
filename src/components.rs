use bevy::{prelude::Component, math::{Vec2, Vec3}};
#[derive(Component)]
pub struct Acc{
    pub acc: Vec3,
    pub vel: Vec3,
}

#[derive(Component)]
pub struct Force{
    pub size: f64,
    pub dir: Vec3,
}

#[derive(Component)]
pub struct Mass{
    pub mass: f64,    
}

#[derive(Component)]
pub struct CircleMesh{
    pub radius: f64,    
}

#[derive(Component)]
pub struct LineMesh{
    pub start: Vec3,
    pub end: Vec3,
    pub width: f64,
}

#[derive(Component)]
pub struct Drag{
    pub drag_coefficient: f64,
    pub fluid_density: f64,
    pub fluid_vel: Vec3,
}


// components in this module allow systems that have overlapping components to be properly identified
pub mod system_identity_components {
    use bevy::prelude::Component;

    // allows entities to be movable
    // required components:
    //  Acc
    #[derive(Component)]
    pub struct Movable;
    
    #[derive(Component)]
    pub struct Force;

    // allows entities to be experience drag based on the fluid's density and relative speed
    // required components:
    //  Acc
    //  Mass
    #[derive(Component)]
    pub struct ExperienceDrag;

    // allows entities to experience gravity
    // required components:
    //  Acc
    //  Mass
    #[derive(Component)]
    pub struct HasGravity;
}
