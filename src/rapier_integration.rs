use nalgebra::{Point, Point3, Vector};
use rapier3d::prelude::*;
use tiny_game_framework::{glam::Vec3, rand_betw, Vertex};

pub struct RapierPhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: (),

    pub received_delta_time: Option<f32>,

    pub handles: Vec<RigidBodyHandle>,
}

impl RapierPhysicsWorld {
    pub fn new() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let ground_collider = ColliderBuilder::cuboid(25.0, 0.1, 25.0).build();
        collider_set.insert(ground_collider);

        let mut handles = vec![];

        let gravity = vector![0.0, -9.8, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        Self {
            rigid_body_set,
            collider_set,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            handles,

            received_delta_time: Some(0.032),
        }
    }

    pub async fn step(&mut self) {
        self.integration_parameters.dt = self.received_delta_time.unwrap();

        self.physics_pipeline.step(
            &vector![0.0, -9.81, 0.0], // gravity
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.received_delta_time = Some(dt);
    }    

    pub fn add_capsule_rigidbody(&mut self, x: f32, y: f32, z: f32) -> RigidBodyHandle {
        let capsule_rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y, z])
            .build();
        let capsule_collider = ColliderBuilder::capsule_y(0.5, 0.5).restitution(0.5).friction(1.0).build();
        let capsule_body_handle = self.rigid_body_set.insert(capsule_rigid_body.clone());

        self.handles.push(capsule_body_handle.clone());
        self.collider_set.insert_with_parent(capsule_collider.clone(), capsule_body_handle, &mut self.rigid_body_set);

        return capsule_body_handle;
    }

    pub fn add_cube_rigidbody(&mut self, x: f32, y: f32, z: f32) -> RigidBodyHandle {
        // i ain't bothering renaming stuff now

        let capsule_rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y, z])
            .build();
        let capsule_collider = ColliderBuilder::cuboid(0.3, 0.3, 0.3).restitution(0.7).friction(0.5).build();
        let capsule_body_handle = self.rigid_body_set.insert(capsule_rigid_body.clone());

        self.handles.push(capsule_body_handle.clone());
        self.collider_set.insert_with_parent(capsule_collider.clone(), capsule_body_handle, &mut self.rigid_body_set);

        return capsule_body_handle;
    }

    pub fn add_sphere_rigidbody(&mut self, x: f32, y: f32, z: f32) -> RigidBodyHandle {
        // i ain't bothering renaming stuff now

        let capsule_rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y, z])
            .build();
        let capsule_collider = ColliderBuilder::ball(0.5).restitution(0.7).friction(0.5).build();
        let capsule_body_handle = self.rigid_body_set.insert(capsule_rigid_body.clone());

        self.handles.push(capsule_body_handle.clone());
        self.collider_set.insert_with_parent(capsule_collider.clone(), capsule_body_handle, &mut self.rigid_body_set);

        return capsule_body_handle;
    }

    pub fn add_static_cube_rigidbody(&mut self, x: f32, y: f32, z: f32) -> RigidBodyHandle {
        // i ain't bothering renaming stuff now
        
        let capsule_rigid_body = RigidBodyBuilder::fixed()
            .translation(vector![x, y, z])
            .build();
        let capsule_collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).restitution(0.7).friction(3.0).build();
        let capsule_body_handle = self.rigid_body_set.insert(capsule_rigid_body.clone());

        self.handles.push(capsule_body_handle.clone());
        self.collider_set.insert_with_parent(capsule_collider.clone(), capsule_body_handle, &mut self.rigid_body_set);

        return capsule_body_handle;
    }

    pub fn remove_rigidbody(&mut self, handle: RigidBodyHandle) {
        self.rigid_body_set.remove(
            handle, 
            &mut self.island_manager, 
            &mut self.collider_set, 
            &mut self.impulse_joint_set, 
            &mut self.multibody_joint_set, 
            true,
        );
    }

    pub fn build_collider_from_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, x: f32, y: f32, z: f32) -> RigidBodyHandle {
        let trimesh = SharedShape::trimesh(
            vertices.iter().map(|v| Point3::new(v.position.x, v.position.y, v.position.z)).collect(),
            indices.chunks(3).map(|c| [c[0] as u32, c[1] as u32, c[2] as u32]).collect()
        );
        
        let mesh_rigid_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![x, y, z])
            .build();
        let mesh_collider = ColliderBuilder::new(trimesh).build();
        let mesh_body_handle = self.rigid_body_set.insert(mesh_rigid_body.clone());
    
        self.handles.push(mesh_body_handle.clone());
        self.collider_set.insert_with_parent(mesh_collider.clone(), mesh_body_handle, &mut self.rigid_body_set);
    
        mesh_body_handle
    }
    
}
