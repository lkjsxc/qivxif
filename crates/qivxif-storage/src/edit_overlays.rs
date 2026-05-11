use crate::{
    errors::{StoreError, redb_error},
    tables::SECTIONS,
};
use qivxif_core::ChunkCoord;
use qivxif_protocol::BlockCell;
use redb::{Database, Durability, ReadableDatabase};

pub(crate) fn load_chunk_overlay(
    db: &Database,
    coord: ChunkCoord,
) -> Result<Vec<BlockCell>, StoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(SECTIONS).map_err(redb_error)?;
    match table
        .get(chunk_overlay_key(coord).as_str())
        .map_err(redb_error)?
    {
        Some(value) => Ok(postcard::from_bytes(value.value())?),
        None => Ok(Vec::new()),
    }
}

pub(crate) fn put_chunk_overlay(
    db: &Database,
    coord: ChunkCoord,
    cells: &[BlockCell],
) -> Result<(), StoreError> {
    let mut write = db.begin_write().map_err(redb_error)?;
    write
        .set_durability(Durability::Immediate)
        .map_err(redb_error)?;
    {
        let mut table = write.open_table(SECTIONS).map_err(redb_error)?;
        let bytes = postcard::to_stdvec(cells)?;
        table
            .insert(chunk_overlay_key(coord).as_str(), bytes.as_slice())
            .map_err(redb_error)?;
    }
    write.commit().map_err(redb_error)?;
    Ok(())
}

pub(crate) fn replace_cell(cells: &mut Vec<BlockCell>, edit: BlockCell) {
    cells.retain(|cell| cell.pos != edit.pos);
    cells.push(edit);
}

pub(crate) fn chunk_overlay_key(coord: ChunkCoord) -> String {
    format!("section/{}/{}", coord.x, coord.z)
}
