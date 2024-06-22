use std::{sync::Arc, time::{Duration, Instant}, sync::Mutex as StdMutex};

use character_controller::Player;
use generation::Canvas;

use generator::{gen_maze_async, new_quadrant};
use once_cell::sync::Lazy;
use rapier_integration::RapierPhysicsWorld;
use tiny_game_framework::{
    gl::{Clear, ClearColor, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT}, glam::{vec2, vec3, vec4, Vec3, Vec3A, Vec4}, glfw::{self, Key}, rand_vec3, Cuboid, EventLoop, Light, Quad, Renderer, Sphere
};
use tokio::sync::{mpsc, Mutex};

mod generation;
mod generator;
mod rapier_integration;
mod character_controller;

const GRAVITY: f32 = 10.;

#[tokio::main]
async fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();
    let mut rapier_world = RapierPhysicsWorld::new();

    renderer.add_texture("test".to_string(), "src/images/tex.png".to_string());
    renderer.add_light("l1", Light { color: Vec3::ONE, position: vec3(1.0, 1.0, 1.0)});
    
    el.window.set_cursor_mode(glfw::CursorMode::Disabled);

    let mut player_mesh = Cuboid::new(vec3(100.0, 100.0, 100.0), vec4(1., 1., 1., 1.)).mesh();
    player_mesh.set_texture("test", &renderer);
    player_mesh.set_shader_type(&tiny_game_framework::ShaderType::Full);
    player_mesh.setup_mesh();
    renderer.add_mesh("player", player_mesh).unwrap();

    let mut receiver = new_quadrant(); // generate new maze quadrant
    
    // defining game state variables ~~~~~
    // ~~~~~

    let mut player = Player::setup(&mut rapier_world, &mut renderer);

    while !el.window.should_close() {
        el.update();
        
        gen_maze_async(&mut receiver, &mut renderer, &mut rapier_world).await;
        
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.input(&el.window, &el.window.glfw);
        
        
        let frame = el.ui.frame(&mut el.window);
        
        
        frame.text("hello, world!");
        
        player.update(&mut rapier_world, &mut el, &mut renderer);
        rapier_world.set_dt(el.dt);
        
        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            ClearColor(0.1, 0.2, 0.3, 1.0);
            renderer.draw(&el);
            el.ui.draw();
        }
        
        let mut move_vec = Vec3::ZERO;
        if el.is_key_down(Key::W){
            move_vec += renderer.camera.front;
        }
        if el.is_key_down(Key::S){
            move_vec -= renderer.camera.front;
        }
        if el.is_key_down(Key::A){
            move_vec -= renderer.camera.front.cross(vec3(0.0, 1.0, 0.0));
        }
        if el.is_key_down(Key::D){
            move_vec += renderer.camera.front.cross(vec3(0.0, 1.0, 0.0));
        }
        // move_vec.y += GRAVITY;
        
        let player_mesh = renderer.get_mesh_mut("player").unwrap();
        player_mesh.position = player.pos.into();
        let pos = player_mesh.position;
        renderer.camera.update((pos + renderer.camera.front * 10.0) / resolution.x);
        
        player.pos = (vec3(player.pos.x, player.pos.y, player.pos.z) + move_vec).into();
        rapier_world.step().await;
        
        if el.is_key_down(Key::LeftAlt) {
            el.window.set_cursor_mode(glfw::CursorMode::Normal);
        }
        else {
            el.window.set_cursor_mode(glfw::CursorMode::Disabled);
        }
    }
    
}
