use bevy::{prelude::Plugin};

pub struct TimePlugin;
impl Plugin for TimePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(functions::calc_delta);
    }
}

mod components{
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct TimeStep{
        pub time_step: f32,
        pub delta_time: f32,
    }
}

mod functions{
    use bevy::{core::Time, prelude::{Query, Res, With}};

    use super::components::TimeStep;

    pub fn calc_delta(
        time: Res<Time>,
        mut q: Query<&mut TimeStep, With<TimeStep>>,
    ) {        
        for mut e in q.iter_mut() {
            e.time_step = time.delta_seconds();            
        }
    }
}