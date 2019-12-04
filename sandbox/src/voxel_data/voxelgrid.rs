use log::{debug, error, info, trace, warn};

use edocore::math::vector::{UVector3};
use crate::utils::rle_array::RLEArray;

pub struct VoxelGrid {
    //pub voxels: [[[u8; 512]; 512]; 512]
    pub voxels: RLEArray<u8>,
    pub size: UVector3,
}

impl VoxelGrid {
    pub fn new(size: UVector3) -> VoxelGrid {
        let mut voxels = RLEArray::with_capacity(0, (size.x as usize)*(size.y as usize)*(size.z as usize));

        VoxelGrid {
            voxels: voxels,
            size: size
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) {

    }
}
