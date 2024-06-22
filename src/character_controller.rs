use std::collections::HashMap;

use rapier3d::{dynamics::RigidBodyHandle, na::vector, parry::query::Ray};
use tiny_game_framework::{glam::{quat, vec3, vec3a, vec4, Mat4, Quat, Vec3, Vec3A}, rand_betw, Cuboid as Goud, EventLoop, Light, Renderer, Sphere};
use tokio::sync::MutexGuard;

use crate::rapier_integration::RapierPhysicsWorld;

use rapier3d::prelude::*;

pub struct Player {
    pub pos: Vec3A,
    collider_handle: RigidBodyHandle,
}

impl Player {
    pub fn setup(rw: &mut RapierPhysicsWorld, r: &mut Renderer) -> Self {
        let pos = vec3a(0.0, 0.0, 0.0);
        let handle = rw.add_capsule_rigidbody(pos.x, pos.y, pos.z);

        rw.rigid_body_set[handle].lock_rotations(false, false); // so it doesnt fall
 
        Self {
            pos,
            collider_handle: handle,
        }
    }

    pub fn update(
        &mut self, 
        rw: &mut RapierPhysicsWorld, 
        el: &mut EventLoop, 
        r: &mut Renderer,
    ) {
        let capsule = &mut rw.rigid_body_set[self.collider_handle];
        
        capsule.set_translation(vector![self.pos.x, self.pos.y, self.pos.z], true);
    
        // capsule.set_translation(vector![self.pos.x, self.pos.y, self.pos.z], true);
    }
}
