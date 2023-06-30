/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

mod bounds;
mod scene;
mod linalg;
mod shaders;
mod model;
mod texture;
mod transform;

use env_logger;

use glium::glutin::event::{ *, VirtualKeyCode as Key, };
use glium::glutin::event_loop;
use glium::glutin::window::CursorGrabMode;
use glium::Surface;

use linalg::{ Scalar as S, Matrix as M, Vector as V, };

use scene::Scene;

use std::f32::consts::FRAC_PI_2;

const TIME_PER_TICK: std::time::Duration
  = std::time::Duration::from_nanos(16_666_667);

/******************************************************************************/
/* acme */

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    a_position: [f32; 3],
    a_normal:   [f32; 3],
}

glium::implement_vertex!(Vertex, a_position, a_normal);

/******************************************************************************/
/* state */

fn update_movement(mov: &V<3>, dir: &V<3>, vel: f32, dt: std::time::Duration)
    -> V<3>
{
    let dt = S(dt.as_secs_f32());
    dt * ( &(S(0.9) * mov) + &(S(vel) * dir) )
}

struct State {
    display: glium::Display,
    keys_pressed: std::collections::HashSet<Key>,
    keys_released: std::collections::HashSet<Key>,
    keys_down: std::collections::HashSet<Key>,
    last_updated: std::time::Instant,
    model: model::Model,
    shape: usize,
    position: V<3>,
    movement: V<3>,
    rx: f32,
    ry: f32,
    view_matrix: M<4,4>,
    projection_matrix: M<4,4>,
}

impl State {
    fn key_down( &self, key: &Key ) -> bool { self.keys_down.contains(key) }
    fn key_up( &self, key: &Key ) -> bool { !self.keys_down.contains(key) }
    fn key_pressed( &self, key: &Key ) -> bool { self.keys_pressed.contains(key) }
    fn key_released( &self, key: &Key ) -> bool { self.keys_released.contains(key) }

    fn draw( &self ) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        if self.shape == self.model.shapes.len() {
            for shape in &self.model.shapes {
                shape.draw(
                    &mut target,
                    &self.projection_matrix,
                    &self.view_matrix
                )
            }
        } else {
            self.model.shapes[self.shape]
                . draw(&mut target, &self.projection_matrix, &self.view_matrix);
        }

        target.finish().unwrap();
    }

    fn update( &mut self, dt: std::time::Duration ) {
        let mut dir = V([0.0, 0.0, 0.0]);

        if self.key_pressed(&Key::J) {
            if self.shape == self.model.shapes.len() {
                self.shape = 0;
            } else {
                self.shape += 1;
            }
        }
        if self.key_pressed(&Key::K) {
            if self.shape == 0 {
                self.shape = self.model.shapes.len();
            } else {
                self.shape -= 1;
            }
        }

        if self.key_down(&Key::D)
            { dir.0[0] += 1.0; }
        if self.key_down(&Key::A)
            { dir.0[0] -= 1.0; }
        if self.key_down(&Key::Space)
            { dir.0[1] += 1.0; }
        if self.key_down(&Key::C)
            { dir.0[1] -= 1.0; }
        if self.key_down(&Key::S)
            { dir.0[2] += 1.0; }
        if self.key_down(&Key::W)
            { dir.0[2] -= 1.0; }

        if self.key_down(&Key::Left)
            { self.ry += 0.01; }
        if self.key_down(&Key::Right)
            { self.ry -= 0.01; }
        if self.key_down(&Key::Up)
            { self.rx += 0.01; }
        if self.key_down(&Key::Down)
            { self.rx -= 0.01; }

        self.rx
          = self.rx.clamp(-FRAC_PI_2, FRAC_PI_2);

        let ry = { let (c,s) = (self.ry.cos(), self.ry.sin());
                   M([[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]]) };
        let rx = { let (c,s) = (self.rx.cos(), self.rx.sin());
                   M([[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]]) };
        let r = &ry * &rx;

        let forward = &r * &V([0.0, 0.0, 1.0]);
        let    up   = &r * &V([0.0, 1.0, 0.0]);

        if let Some(ndir) = dir.normalize()
            { dir = &ry * &ndir; }

        let sprint = self.key_down(&Key::LShift);
        let speed = if sprint {3.0} else {2.0};

        self.movement = update_movement(&self.movement, &dir, speed, dt);
        self.position = &self.position + &self.movement;

        self.view_matrix = transform::view(&self.position, &forward, &up);

        let (w, h) = self.display.get_framebuffer_dimensions();
        self.projection_matrix = transform::field_of_view_deg
            ( 0.1, 1024.0, w as f32, h as f32, 90.0 );
    }

    fn handle_keyboard_input( &mut self, input: KeyboardInput ) {
        match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(k),
                .. }
            =>  {
                self.keys_pressed.insert(k);
                self.keys_down.insert(k);
            },

            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(k),
                .. }
            =>  {
                self.keys_released.insert(k);
                self.keys_down.remove(&k);
            },

            _
            =>  (),
        }
    }

    fn handle_mouse_motion( &mut self, (x, y): (f64, f64) ) {
        let half_pi = 0.5 * std::f32::consts::PI;
        self.ry -= x as f32 * 0.001;
        self.rx = (self.rx - 0.001 * y as f32).clamp(-half_pi, half_pi);
    }
}

/******************************************************************************/
/* main */

fn main() {
    env_logger::init();
    let main_loop = event_loop::EventLoop::new();
    let display = glium::Display::new(
        glium::glutin::window::WindowBuilder::new(),
        glium::glutin::ContextBuilder::new().with_depth_buffer(24),
        &main_loop
        ).unwrap();
    let _ = display.gl_window().window().set_cursor_grab(CursorGrabMode::Confined);
    display.gl_window().window().set_cursor_visible(false);

    let mut state = State {
        keys_down: std::collections::HashSet::new(),
        keys_pressed: std::collections::HashSet::new(),
        keys_released: std::collections::HashSet::new(),
        last_updated: std::time::Instant::now(),
        model: model::obj::load(&display, "assets/OBJ/default.obj").unwrap(),
        shape: 0,
        display: display,
        position: V([0.0, 0.0, 0.0]),
        movement: V([0.0, 0.0, 0.0]),
        rx: 0.0,
        ry: 0.0,
        view_matrix: M::identity(),
        projection_matrix: M::identity(),
    };

    let mut frames: usize = 0;
    let mut next_sec = std::time::Instant::now()
                     + std::time::Duration::from_secs(1);

    /**************************************************************************/
    /* main loop */

    main_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        match event {
            Event::WindowEvent { event, .. }
            =>  match event {
                WindowEvent::CloseRequested
                =>  *control_flow = event_loop::ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. }
                =>  state.handle_keyboard_input( input ),

                _
                =>  (),
            },

            Event::DeviceEvent { event, .. }
            =>  match event {
                DeviceEvent::MouseMotion{ delta }
                =>  state.handle_mouse_motion( delta ),

                _
                =>  (),
            },

            Event::NewEvents(cause)
            =>  match cause {
                StartCause::Init
                =>  *control_flow
                  = event_loop::ControlFlow::WaitUntil(now + TIME_PER_TICK),

                StartCause::ResumeTimeReached { .. }
                =>  *control_flow
                  = event_loop::ControlFlow::WaitUntil(now + TIME_PER_TICK),

                _
                =>  (),
            },

            Event::MainEventsCleared
            =>  {
                let time_elapsed = state.last_updated.elapsed();
                let update = time_elapsed >= TIME_PER_TICK;

                if update {
                    state.update(time_elapsed);
                    state.last_updated = now;
                }

                state.draw();

                if update {
                    state.keys_pressed = std::collections::HashSet::new();
                    state.keys_released = std::collections::HashSet::new();
                }

                if now >= next_sec {
                    next_sec = now + std::time::Duration::from_secs(1);
                    println!("fps: {frames}");
                    frames = 0;
                }
                frames += 1;
            },

            Event::RedrawEventsCleared
            =>  (),

            _
            =>  (),
        }
    });
}
