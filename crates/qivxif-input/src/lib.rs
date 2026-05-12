use qivxif_core::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    ThirdPerson,
    FirstPerson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelTarget {
    pub pos: BlockPos,
    pub place_pos: BlockPos,
}

pub fn target_from_origin(origin: BlockPos, forward: BlockPos) -> VoxelTarget {
    VoxelTarget {
        pos: origin,
        place_pos: BlockPos {
            x: origin.x + forward.x.signum(),
            y: origin.y + forward.y.signum(),
            z: origin.z + forward.z.signum(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_place_pos_uses_direction_sign() {
        let target = target_from_origin(
            BlockPos { x: 2, y: 3, z: 4 },
            BlockPos { x: -9, y: 0, z: 8 },
        );
        assert_eq!(target.place_pos, BlockPos { x: 1, y: 3, z: 5 });
    }
}
