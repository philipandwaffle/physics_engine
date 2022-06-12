mod acceleration{
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

mod force{
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

mod time{
    use bevy::{prelude::Component};

    #[derive(Component)]
    pub struct TimeStep{
        pub time_step: f32,
        pub delta_time: f32,
    }
}

mod mass {    
    use bevy::{prelude::Component};

    #[derive(Component)]
    pub struct Mass{
        pub mass: f32,    
    }
}

mod mesh{
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