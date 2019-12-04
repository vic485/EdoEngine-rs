use log::{debug, error, info, trace, warn};

use crate::utils::rle_array::RLEArray;

pub struct VoxelGrid {
    //pub voxels: [[[u8; 512]; 512]; 512]
    pub voxels: RLEArray<u8>
}

impl VoxelGrid {
    pub fn new() -> VoxelGrid {
        let mut voxels = RLEArray::new();

        for i in 0..8 {
            voxels.push(8);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        for i in 0..3 {
            voxels.push(2);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        for i in 0..5 {
            voxels.push(8);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        for i in 0..2 {
            if let Err(why) = voxels.remove(8) {
                error!("ERROR: {}", why);
            }
            // debug!("Value at index 14: {}", voxels.get(8).unwrap());
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        if let Err(why) = voxels.remove(13) {
            error!("ERROR: {}", why);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        debug!("Attempting to set index 4 to 1");
        if let Err(why) = voxels.set(1, 4) {
            error!("ERROR: {}", why);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        debug!("Attempting to insert 9 into index 7");
        if let Err(why) = voxels.insert(9, 7) {
            error!("ERROR: {}", why);
        }

        debug!("Voxel table length: {}", voxels.len());
        debug!("Table contents: {}", voxels.get_pretty_string());

        VoxelGrid {
            voxels: voxels
        }
    }
}
