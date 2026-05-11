use super::*;

#[tokio::test]
async fn actor_serializes_mutation() {
    let root = tempfile::tempdir().unwrap();
    let store = Arc::new(WorldStore::open(root.path(), 3).unwrap());
    let region = RegionHandle::spawn(3, store);
    let pos = BlockPos { x: 1, y: 3, z: 1 };
    region.place_block(pos, 8).await.unwrap();
    let cells = region.chunk(ChunkCoord { x: 0, z: 0 }).await.unwrap();
    assert!(cells.contains(&BlockCell { pos, block: 8 }));
}

#[tokio::test]
async fn mutation_flushes_to_store() {
    let root = tempfile::tempdir().unwrap();
    let store = Arc::new(WorldStore::open(root.path(), 3).unwrap());
    let region = RegionHandle::spawn(3, store.clone());
    let pos = BlockPos { x: -1, y: 3, z: -1 };
    region.place_block(pos, 8).await.unwrap();
    assert!(
        store
            .load_chunk(ChunkCoord { x: -1, z: -1 })
            .unwrap()
            .is_empty()
    );
    region.flush().await.unwrap();
    assert_eq!(
        store.load_chunk(ChunkCoord { x: -1, z: -1 }).unwrap(),
        vec![BlockCell { pos, block: 8 }]
    );
}

#[tokio::test]
async fn invalid_y_is_rejected() {
    let root = tempfile::tempdir().unwrap();
    let store = Arc::new(WorldStore::open(root.path(), 3).unwrap());
    let region = RegionHandle::spawn(3, store);
    let pos = BlockPos { x: 1, y: 99, z: 1 };
    assert!(matches!(
        region.place_block(pos, 8).await,
        Err(SimError::InvalidBlockPos(_))
    ));
}
