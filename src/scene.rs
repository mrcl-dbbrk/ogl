/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::M;

pub trait Scene {
    fn draw(
        &self,
        target: &mut glium::Frame,
        perspective_matrix: &M<4,4>,
        model_view_matrix: &M<4,4>,
    );
}

pub struct SceneTree<'a> {
    children: Vec<&'a dyn Scene>
}

impl Scene for SceneTree<'_> {
    fn draw( &self,
             target: &mut glium::Frame,
             perspective_matrix: &M<4,4>,
             model_view_matrix: &M<4,4>, )
    {
        for scene in self.children.iter()
          { scene.draw(target, perspective_matrix, model_view_matrix); }
    }
}

mod light {
    pub struct AmbientLight {
        color: [f32; 3],
    }
    pub struct DirectionalLight {
        color: [f32; 3],
        direction: [f32; 3],
    }
    pub struct PointLight {
        color: [f32; 3],
        position: [f32; 3],
    }
    pub struct Lights {
        ambient: AmbientLight,
        directional: DirectionalLight,
        point: PointLight,
    }
}
