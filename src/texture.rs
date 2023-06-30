/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

pub fn load( display: &glium::Display, file_path: &str )
-> glium::texture::SrgbTexture2d
{
    let img = image::open( file_path ).unwrap().to_rgba8();
    let dim = img.dimensions();

    let raw_img = glium::texture::RawImage2d
      ::from_raw_rgba_reversed( &img.into_raw(), dim );

    glium::texture::SrgbTexture2d::new( display, raw_img ).unwrap()
}
