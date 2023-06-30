/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

use std::ops::{Add, Sub, Mul};
use std::iter;
use std::iter::zip;
use std::array::from_fn as arr;

/******************************************************************************/
/* Scalar */

pub struct Scalar(pub f32);

impl From<f32> for Scalar {
    fn from(a: f32) -> Scalar {
        Scalar(a)
    }
}

impl Mul for Scalar {
    type Output = Scalar;
    fn mul(self, rhs: Scalar) -> Scalar
      { Scalar(self.0 * rhs.0) }
}
impl Mul<&Scalar> for Scalar {
    type Output = Scalar;
    fn mul(self, rhs: &Scalar) -> Scalar
      { Scalar(self.0 * rhs.0) }
}
impl Mul for &Scalar {
    type Output = Scalar;
    fn mul(self, rhs: &Scalar) -> Scalar
      { Scalar(self.0 * rhs.0) }
}
impl Mul<Scalar> for &Scalar {
    type Output = Scalar;
    fn mul(self, rhs: Scalar) -> Scalar
      { Scalar(self.0 * rhs.0) }
}

impl <const N: usize> Mul<Vector<N>> for Scalar {
    type Output = Vector<N>;
    fn mul(self, rhs: Vector<N>) -> Vector<N>
      { Vector( arr(|n| self.0 * rhs.0[n]) ) }
}
impl <const N: usize> Mul<&Vector<N>> for Scalar {
    type Output = Vector<N>;
    fn mul(self, rhs: &Vector<N>) -> Vector<N>
      { Vector( arr(|n| self.0 * rhs.0[n]) ) }
}
impl <const N: usize> Mul<Vector<N>> for &Scalar {
    type Output = Vector<N>;
    fn mul(self, rhs: Vector<N>) -> Vector<N>
      { Vector( arr(|n| self.0 * rhs.0[n]) ) }
}
impl <const N: usize> Mul<&Vector<N>> for &Scalar {
    type Output = Vector<N>;
    fn mul(self, rhs: &Vector<N>) -> Vector<N>
      { Vector( arr(|n| self.0 * rhs.0[n]) ) }
}

/******************************************************************************/
/* Vector */

#[derive(Debug)]
pub struct Vector<const N: usize>
  ( pub [f32; N] );

impl <const N: usize>
Vector<N> {
    pub fn normalize<>(&self) -> Option<Self> {
        let len_len = self * self;
        if len_len == 0.0
          { None }
        else
          { Some( Scalar(1.0/len_len.sqrt()) * self ) }
    }
}

impl <const N: usize>
Mul for &Vector<N> {
    type Output = f32;
    fn mul(self, rhs: Self) -> f32
      { iter::zip(self.0, rhs.0).map(|(a,b)| a*b).reduce(|a,b| a+b).unwrap() }
}

//impl <const N: usize>
//Mul<f32> for &Vector<N> {
//    type Output = Vector<N>;
//    fn mul(self, rhs: f32) -> Vector<N>
//      { Vector( arr(|n| self.0[n] * rhs) ) }
//}

impl <const N: usize>
Add for &Vector<N> {
    type Output = Vector<N>;
    fn add(self, rhs: Self) -> Vector<N>
      { Vector( arr(|n| self.0[n] + rhs.0[n]) ) }
}

impl <const N: usize>
Sub for &Vector<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Self) -> Vector<N>
      { Vector( arr(|n| self.0[n] - rhs.0[n]) ) }
}

/******************************************************************************/
/* Matrix */

#[derive(Debug)]
pub struct Matrix<const M: usize, const N: usize>
  ( pub [[f32; M]; N] );

impl <const M: usize, const N: usize>
Matrix<M,N> {
    pub fn transpose(&self) -> Matrix<N,M>
      { Matrix( arr(|m| arr(|n| self.0[n][m])) ) }

    pub fn upper_left<const O: usize, const P: usize> (&self) -> Matrix<O,P>
      { Matrix( arr(|p| arr(|o| self.0[p][o])) ) }
}

impl <const N: usize>
Matrix<N,N> {
    pub fn det(&self) -> f32 {
        (0..N) . map( |n| zip( 0..N, (n..N).chain(0..n) )
                          . map( |(m, n)| self.0[n][m] )
                          . product::<f32>()
                        - zip( (0..N).rev(), (n..N).chain(0..n) )
                          . map( |(m, n)| self.0[n][m] )
                          . product::<f32>() )
        . sum()
    }

    pub fn identity() -> Self {
        let mut m = Matrix([[0.0; N]; N]);
        for n in 0..N {m.0[n][n] = 1.0}
        m
    }
}

pub fn cross<const N: usize>(vectors: [&Vector<N>; N-1]) -> Vector<N> {
    Vector( arr( |n| zip( 0..N-1, (n+1..N).chain(0..n) )
                     . map( |(m, n)| vectors[m].0[n] )
                     . product::<f32>()
                   - zip( (0..N-1).rev(), (n+1..N).chain(0..n) )
                     . map( |(m, n)| vectors[m].0[n] )
                     . product::<f32>() ) )
}

impl <const L: usize, const M: usize, const N: usize>
Mul<&Matrix<M,N>> for &Matrix<L,M> {
    type Output = Matrix<L,N>;
    fn mul(self, rhs: &Matrix<M,N>) -> Matrix<L,N> {
        let lhs = self.transpose();
        Matrix( arr(|n| arr(|l| &Vector(lhs.0[l]) * &Vector(rhs.0[n]))) )
    }
}

impl <const L: usize, const M: usize>
Mul<&Vector<M>> for &Matrix<L,M> {
    type Output = Vector<L>;
    fn mul(self, rhs: &Vector<M>) -> Vector<L> {
        let lhs = self.transpose();
        Vector( arr(|l| &Vector(lhs.0[l]) * rhs))
    }
}
