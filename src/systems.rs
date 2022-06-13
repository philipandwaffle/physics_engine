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
        for (mut transform, _movable, mut acc) in query.iter_mut(){
            //let delta_time = time_step.time.delta_seconds();
            acc.vel = acc.vel.add(acc.acc.mul(time_step.time_step));
            transform.translation.x += acc.vel.x;
            transform.translation.y += acc.vel.y;

            println!("{}",acc.vel);
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
        for (mut acc, drag, mass, _exp_drag) in query.iter_mut(){
            if acc.vel != vec2(0., 0.){
                let drag_force = drag.drag_coefficient * ((drag.fluid_density * (drag.fluid_vel - acc.vel)) / 2.);
                let drag_vel = drag_force / mass.mass;

                println!("{}",drag_vel);

                if acc.vel.x.abs() < drag_vel.x.abs(){
                    acc.vel.x = 0.;
                }else if acc.vel.y.abs() < -drag_vel.y.abs(){
                    acc.vel.y = 0.;
                }else{
                    acc.vel += drag_vel * time_step.time_step;   
                }                
            }
        }
    }
}

pub mod gravity_system{
    use std::vec;

    use bevy::{prelude::{Query, Res, Entity}, math::{vec2, Vec2}};
    use crate::{components::{mass::{Mass, self}, acceleration::Acc2, system_identity_components::HasGravity}, TimeStep, GravitationalConstant};

    pub fn gravity(
        time_step: Res<TimeStep>,
        g_const: Res<GravitationalConstant>,
        mut query: Query<(Entity, &mut Acc2, &Mass, &HasGravity)>
    ){
        let mut entities: Vec<(Entity, f32, Vec2)> = vec![];       
        
        for (entity, _acc, mass, _gravity) in query.iter(){
            entities.push((entity, mass.mass, vec2(0., 0.)));
        }

        let mut i = 0;
        entities.iter_mut().for_each(|e|{
            
        });
        let f = entities.get(0).unwrap().0;
        query.get(entities.get(0).unwrap().0);
    }
}