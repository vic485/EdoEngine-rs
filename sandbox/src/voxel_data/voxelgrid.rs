use log::{debug, error, info, trace, warn};

use crate::utils::rle_array::RLEArray;

pub struct VoxelGrid {
    //pub voxels: [[[u8; 512]; 512]; 512]
    pub voxels: RLEArray<u8>
}

impl VoxelGrid {
    pub fn new() -> VoxelGrid {
        // let mut voxels = RLEArray::new();
        //
        // for i in 0..(512*512*512) {
        //     voxels.push(4);
        // }

        let mut voxels = RLEArray::with_capacity(0, 512*512*512);

        VoxelGrid {
            voxels: voxels
        }
    }
}
