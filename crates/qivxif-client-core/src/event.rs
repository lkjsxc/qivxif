use qivxif_core::ChunkCoord;
use qivxif_protocol::BlockCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeEvent {
    Connected { world_id: String },
    Joined { player: String },
    ChunkLoaded { coord: ChunkCoord, cells: usize },
    MutationApplied { cell: BlockCell },
}
