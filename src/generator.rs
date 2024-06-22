use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::{mpsc::Receiver, Mutex};

use tiny_game_framework::{glam::{vec3, Vec3, Vec4}, Cuboid, Renderer, Vertex};
use tokio::sync::mpsc;

use crate::{generation::Canvas, rapier_integration::RapierPhysicsWorld};

static GLOBAL_MESH_COUNTER: Lazy<Arc<Mutex<usize>>> = Lazy::new(|| {
    Arc::new(Mutex::new(0))
});

pub struct MeshResult {
    pub shape: Cuboid,
    pub position: Vec3,
}

pub fn new_quadrant() -> Receiver<MeshResult> {
    let (sender, receiver) = mpsc::channel::<MeshResult>(1);
    
    tokio::spawn(async move {
        let mut canvas = Canvas::new(12, 12);

        canvas.write();
        canvas.print();
        
        let pixels = &canvas.pixels;

        for x in 0..pixels.len() {
            for y in 0..pixels[1].len() {
                if pixels[x][y] == [0, 0, 0, 255] {
                    let mesh = Cuboid::new(vec3(200.0, 200.0, 200.0), Vec4::ONE);
                    let position = vec3(x as f32, 0.0, y as f32) * 200.0;

                    sender.send(MeshResult { shape: mesh, position, }).await.unwrap_or_else(|_| {
                        
                    });
                }
            }
        }
        
    });

    return receiver;
}

pub async fn gen_maze_async(receiver: &mut Receiver<MeshResult>, renderer: &mut Renderer, rw: &mut RapierPhysicsWorld) {
    while let Ok(mesh_result) = receiver.try_recv() {
        let MeshResult { shape, position } = mesh_result;
        let mut mesh = shape.mesh();
        mesh.position = position;
        rw.build_collider_from_mesh(mesh.vertices.clone(), mesh.indices.clone(), position.x, position.y, position.z);
        
        mesh.setup_mesh();
    
        let mut global_mesh_counter = GLOBAL_MESH_COUNTER.lock().await;
        renderer.add_mesh(&format!("MAZE_MESH{:?}{:?}{:?}", position.x, position.y, global_mesh_counter), mesh).unwrap();


        *global_mesh_counter += 1;
    }
}