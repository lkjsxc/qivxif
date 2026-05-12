//! Camera, pointer targeting, and client command mapping helpers for native
//! clients.

use qivxif_client_core::RuntimeCommand;
use qivxif_core::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    ThirdPerson,
    FirstPerson,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraState {
    pub center_x: f32,
    pub center_z: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraAction {
    Pan { dx: f32, dz: f32 },
    Zoom { delta: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelTarget {
    pub pos: BlockPos,
    pub place_pos: BlockPos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerTarget {
    pub pos: BlockPos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientAction {
    PlaceSelected,
    RemoveTarget,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_z: 0.0,
            zoom: 16.0,
        }
    }
}

impl CameraState {
    pub fn apply(&mut self, action: CameraAction) {
        match action {
            CameraAction::Pan { dx, dz } => {
                self.center_x += dx / self.zoom.max(1.0);
                self.center_z += dz / self.zoom.max(1.0);
            }
            CameraAction::Zoom { delta } => {
                self.zoom = (self.zoom + delta).clamp(4.0, 96.0);
            }
        }
    }

    pub fn target_from_pointer(
        &self,
        pointer_x: f32,
        pointer_y: f32,
        width: f32,
        height: f32,
    ) -> PointerTarget {
        let world_x = self.center_x + (pointer_x - width * 0.5) / self.zoom.max(1.0);
        let world_z = self.center_z + (pointer_y - height * 0.5) / self.zoom.max(1.0);
        PointerTarget {
            pos: BlockPos {
                x: world_x.round() as i32,
                y: 3,
                z: world_z.round() as i32,
            },
        }
    }
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

pub fn command_for_action(
    action: ClientAction,
    target: PointerTarget,
    selected_block: u16,
) -> RuntimeCommand {
    match action {
        ClientAction::PlaceSelected => RuntimeCommand::Place {
            pos: target.pos,
            block: selected_block,
        },
        ClientAction::RemoveTarget => RuntimeCommand::Remove { pos: target.pos },
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

    #[test]
    fn camera_targets_center_cell() {
        let camera = CameraState::default();
        let target = camera.target_from_pointer(64.0, 64.0, 128.0, 128.0);
        assert_eq!(target.pos, BlockPos { x: 0, y: 3, z: 0 });
    }
}
