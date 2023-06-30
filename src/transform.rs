use crate::linalg::{Matrix, Vector, Scalar, cross};
use std::f32::consts::PI;

//fn ortho( near:f32, far:f32, width:f32, height:f32 ) -> Matrix<4,4> {
//    todo!()
//}

pub fn field_of_view_deg( near:f32, far:f32, width:f32, height:f32,
                      field_of_view:f32 )
    -> Matrix<4,4>
{
    field_of_view_rad( near, far, width, height,
                       field_of_view * PI / 360.0 )
}

fn field_of_view_rad ( near:f32, far:f32, width:f32, height:f32,
                       field_of_view:f32 )
    -> Matrix<4,4>
{
    let right = near * field_of_view.sin();
    let top = right * height / width;
    frustum( near, far, -right, right, -top, top )
}

fn frustum( near:f32, far:f32, left:f32, right:f32, bottom:f32, top:f32 )
    -> Matrix<4,4>
{
    let (n,f,l,r,b,t) = ( near, far, left, right, bottom, top );

    Matrix
    ([[  2.0*n/(r-l)  ,      0.0      ,      0.0      ,      0.0      ],
      [      0.0      ,  2.0*n/(t-b)  ,      0.0      ,      0.0      ],
      [  (r+l)/(r-l)  ,  (t+b)/(t-b)  ,  -(f+n)/(f-n) ,     -1.0      ],
      [      0.0      ,      0.0      , -2.0*f*n/(f-n),      0.0      ]])
}

pub fn view( position: &Vector<3>, forward: &Vector<3>, up: &Vector<3> )
    -> Matrix<4, 4>
{
    let p = Scalar(-1.0) * position;
    let f = forward.normalize().unwrap();
    let s = cross([up, &f]).normalize().unwrap();
    let u = cross([&f, &s]).normalize().unwrap();
    let p = &(Matrix([s.0,u.0,f.0]).transpose()) * &p;

    Matrix([[s.0[0], u.0[0], f.0[0], 0.0],
            [s.0[1], u.0[1], f.0[1], 0.0],
            [s.0[2], u.0[2], f.0[2], 0.0],
            [p.0[0], p.0[1], p.0[2], 1.0],])
}
