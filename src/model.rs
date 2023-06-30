/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    bounds::BoxBounds,
    linalg::{Matrix, Vector},
    scene,
    shaders,
    Vertex,
};
use glium::Surface;
use std::f32::consts::PI;

/******************************************************************************/
/* modules */

pub mod obj {
    use crate::model::{*};
    pub use tobj::LoadError;

    pub fn load( display: &glium::Display, path: &str )
        -> Result<Model, LoadError>
    {
        let load_result = tobj::load_obj( path, &tobj::GPU_LOAD_OPTIONS );

        let (tobj_models, materials_result) = load_result?;
        let materials = materials_result?;

        Ok ( Model { shapes: tobj_models.into_iter()
                               . map( |m| {(display, m, &materials).into()} )
                               . collect() } )
    }

    impl From<&tobj::Mesh>
    for Mesh {
        fn from(src: &tobj::Mesh) -> Self {
            Mesh {
                vertices:
                    src.positions.chunks_exact(3)
                     . zip( src.normals.chunks_exact(3) )
                     . map(|(p, n)| Vertex{ a_position: p.try_into().unwrap(),
                                            a_normal:   n.try_into().unwrap() })
                     . collect(),
                indices: src.indices.clone(),
            }
        }
    }

    impl From<&tobj::Material>
    for Material {
        fn from
        ( &tobj::Material{ diffuse, specular, shininess, .. }: &tobj::Material )
            -> Material
        {
            Material {
                diffuse,
                specular,
                roughness: 2.0 / (2.0 + shininess).sqrt(),
            }
        }
    }

    impl From<(&glium::Display, tobj::Model, &Vec<tobj::Material>)>
    for Shape {
        fn from( (display, tobj_model, materials):
                     (&glium::Display, tobj::Model, &Vec<tobj::Material>) )
            -> Self
        {
            let mesh: Mesh = (&tobj_model.mesh).into();

            let mut bounds = BoxBounds([[0.0; 3]; 2]);
            for position in tobj_model.mesh.positions.chunks_exact(3)
              { bounds.extend(position); }

            let material = if let Some(id) = tobj_model.mesh.material_id
              { (&materials[id]).into() }
            else
              { Default::default() }
            ;

            Shape {
                gpu_buffer: BufferedShape {
                    vertices:
                        glium::VertexBuffer::new( display, &mesh.vertices )
                          . unwrap(),
                    indices:
                        glium::IndexBuffer::new(
                            display,
                            glium::index::PrimitiveType::TrianglesList,
                            &mesh.indices
                        ).unwrap(),
                    program:
                        glium::Program::from_source(
                            display,
                            shaders::VERTEX_SHADER_SRC,
                            shaders::FRAGMENT_SHADER_SRC,
                            None
                        ).unwrap(),
                },
                mesh,
                material,
                bounds,
                transform: Matrix::identity(),
            }
        }
    }
}

/******************************************************************************/
/* Shape */

struct BufferedShape {
    vertices: glium::VertexBuffer<Vertex>,
    indices:  glium::IndexBuffer<u32>,
    program:  glium::Program,
}

struct Mesh {
    vertices: Vec<Vertex>,
    indices:  Vec<u32>,
}

pub struct Material {
    pub diffuse:    [f32; 3],
    pub specular:   [f32; 3],
    pub roughness:  f32,
}

impl Default
for Material {
    fn default() -> Material {
        Material {
            diffuse:   [0.75, 0.75, 0.75],
            specular:  [0.02; 3],
            roughness: 0.5,
        }
    }
}

pub struct Shape {
    gpu_buffer: BufferedShape,
    mesh:       Mesh,
    material:   Material,
    bounds:     BoxBounds<3>,
    transform:  Matrix<4,4>,
}

impl scene::Scene
for Shape {
    fn draw( &self,
             target: &mut glium::Frame,
             perspective_matrix: &Matrix<4,4>,
             model_view_matrix: &Matrix<4,4> )
    {
        const AMBIENT_COLOR: [f32; 3] = [0.1*PI, 0.1*PI, 0.2*PI];
        const DIRECTIONAL_COLOR: [f32; 3] = [0.9*PI, 0.9*PI, 0.8*PI];
        const DIRECTIONAL_DIRECTION: [f32; 3] = [1.0, 1.0, 1.0];
        const POINT_COLOR: [f32; 3] = [PI, PI, PI];
        const POINT_POSITION: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let uniforms = glium::uniforms::UniformsStorage
          ::new("u_model_view_matrix", (model_view_matrix * &self.transform).0)
          . add("u_perspective_matrix", perspective_matrix.0)
          . add("u_lights.ambient.color", AMBIENT_COLOR)
          . add("u_lights.directional.color", DIRECTIONAL_COLOR)
        //rotate directional light into model view space
          . add("u_lights.directional.direction",
                (&model_view_matrix.upper_left::<3,3>()
                  * &Vector( DIRECTIONAL_DIRECTION )).0)
          . add("u_lights.point.color", POINT_COLOR)
        //move point light into model view space
          . add::<[f32; 3]>("u_lights.point.position",
                (model_view_matrix * &Vector( POINT_POSITION )).0[0..3].try_into().unwrap())
          . add("u_diffuse", self.material.diffuse)
          . add("u_specular", self.material.specular)
          . add("u_roughness", self.material.roughness.powi(4))
        ;

        target.draw(
            &self.gpu_buffer.vertices,
            &self.gpu_buffer.indices,
            &self.gpu_buffer.program,
            &uniforms,
            &glium::DrawParameters {
                depth:
                    glium::Depth {
                        test:  glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },

                backface_culling:
                    glium::draw_parameters::BackfaceCullingMode::CullClockwise,

                .. Default::default()
            }
        ).unwrap();
    }
}

/******************************************************************************/
/* Model */

pub struct Model { pub shapes: Vec<Shape>, }

impl scene::Scene
for Model {
  fn draw( &self,
           target: &mut glium::Frame,
           perspective_matrix: &Matrix<4,4>,
           model_view_matrix: &Matrix<4,4> )
  { for shape in self.shapes.iter()
    { shape.draw(target, perspective_matrix, model_view_matrix); } }
}
