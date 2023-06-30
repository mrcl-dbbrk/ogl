/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

/******************************************************************************/
/* BoxBounds */

pub struct BoxBounds<const N: usize> ( pub [[f32; N]; 2] );

impl<const N: usize> BoxBounds<N> {
    pub fn extend(&mut self, point: &[f32]) {
        let [mut bound0, mut bound1] = self.0;
        for i in 0..N {
            if point[i] < bound0[i] {
                bound0[i] = point[i];
            } else if point[i] > bound1[i] {
                bound1[i] = point[i];
            }
        }
    }
}
