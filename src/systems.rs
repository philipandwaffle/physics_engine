pub mod movable_system{
    use std::ops::{Mul, Add};

    use bevy::{prelude::{Query, Transform, Res}};

    use crate::TimeStep;
    use crate::components::acceleration::Acc2;
    use crate::components::system_identity_components::Movable;

    pub fn move_entities(
        time_step: Res<TimeStep>,
        mut query: Query<(&mut Transform, &Movable, &mut Acc2)>
    ){        
        for (mut transform, movable, mut acc) in query.iter_mut(){
            println!("");
            let delta_time = time_step.time.delta_seconds();
            acc.vel = acc.vel.add(acc.acc.mul(time_step.time_step));
            transform.translation.x += acc.vel.x;
            transform.translation.y += acc.vel.y;
        }
    }
}

pub mod drag_system{
    use bevy::{prelude::{Query, Res}, math::vec2};

    use crate::{components::{acceleration::Acc2, drag::Drag2, mass::Mass}, TimeStep};
    use crate::components::system_identity_components::ExperienceDrag;

    pub fn apply_drag(
        time_step: Res<TimeStep>,
        mut query: Query<(&mut Acc2, &Drag2, &Mass, &ExperienceDrag)>
    ){
        for (mut acc, drag, mass, exp_drag) in query.iter_mut(){
            if acc.vel != vec2(0., 0.){
                let drag_force = drag.drag_coefficient * ((drag.fluid_density * (drag.fluid_vel - acc.vel)) / 2.);
                let mut drag_vel = drag_force / mass.mass;

                if acc.vel.x.abs() < drag_vel.x.abs(){
                    drag_vel.x = 0.;
                }else if acc.vel.y.abs() < -drag_vel.y.abs(){
                    drag_vel.y = 0.;
                }

                acc.vel += drag_vel * time_step.time_step;
            }
        }
    }
}