pub mod acceleration{
    use bevy::{prelude::Component, math::{Vec2, Vec3}};
    #[derive(Component)]
    pub struct Acc2{
        pub acc: Vec2,
        pub vel: Vec2,
    }
    #[derive(Component)]
    pub struct Acc3{
        pub acc: Vec3,
        pub vel: Vec3,
    }
}

pub mod force{
    use bevy::{prelude::Component, math::{Vec2, Vec3}};

    #[derive(Component)]
    pub struct Force2{
        pub size: f32,
        pub dir: Vec2,
    }
    #[derive(Component)]
    pub struct Force3{
        pub size: f32,
        pub dir: Vec3,
    }
}

pub mod mass {    
    use bevy::{prelude::Component};

    #[derive(Component)]
    pub struct Mass{
        pub mass: f32,    
    }
}

pub mod mesh{
    use bevy::{prelude::Component, math::{Vec2, Vec3}};

    #[derive(Component)]
    pub struct CircleMesh{
        pub radius: f32,    
    }
    #[derive(Component)]
    pub struct Line2Mesh{
        pub start: Vec2,
        pub end: Vec2,
        pub width: f32,
    }
    #[derive(Component)]
    pub struct Line3Mesh{
        pub start: Vec3,
        pub end: Vec3,
        pub width: f32,
    }
}

pub mod drag{
    use bevy::{prelude::Component, math::{Vec2, Vec3}};

    #[derive(Component)]
    pub struct Drag2{
        pub drag_coefficient: f32,
        pub fluid_density: f32,
        pub fluid_vel: Vec2,
    }
    
    #[derive(Component)]
    pub struct Drag3{
        pub drag_coefficient: f32,
        pub fluid_density: f32,
        pub fluid_vel: Vec3,
    }
}

// components in this module allow systems that have overlapping components to be properly identified
pub mod system_identity_components {
    use bevy::prelude::Component;

    // allows entities to be movable
    // required components:
    //  Acc2 or Acc 3
    #[derive(Component)]
    pub struct Movable;
    
    #[derive(Component)]
    pub struct Force;

    // allows entities to be experience drag based on the fluid's density and relative speed
    // required components:
    //  Acc2 or Acc 3
    //  Mass
    #[derive(Component)]
    pub struct ExperienceDrag;

    // allows entities to experience gravity
    // required components:
    //  Acc2 or Acc 3
    //  Mass
    #[derive(Component)]
    pub struct HasGravity;
}
