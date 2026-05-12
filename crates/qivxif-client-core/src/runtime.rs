use crate::{
    Client, ClientCommand, ClientConfig, HelloReceipt, RuntimeEvent, WorldCache, mutate_block,
};
use anyhow::Result;
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::RequestId;

pub struct ClientRuntime {
    client: Client,
    cache: WorldCache,
    events: Vec<RuntimeEvent>,
    next_request: RequestId,
    world_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSummary {
    pub world_id: String,
    pub chunks: usize,
    pub cells: usize,
}

impl ClientRuntime {
    pub async fn connect_join(config: &ClientConfig, player: &str) -> Result<Self> {
        let client = Client::connect(config).await?;
        let hello = client.hello().await?;
        crate::join_world(&client, player).await?;
        Ok(Self::from_joined(client, hello, player))
    }

    pub async fn fetch_neighborhood(&mut self, center: ChunkCoord, radius: i32) -> Result<()> {
        for x in center.x - radius..=center.x + radius {
            for z in center.z - radius..=center.z + radius {
                let coord = ChunkCoord { x, z };
                let cells = crate::request_chunk(&self.client, coord).await?;
                let count = cells.len();
                self.cache.insert_chunk(coord, cells);
                self.events.push(RuntimeEvent::ChunkLoaded {
                    coord,
                    cells: count,
                });
            }
        }
        Ok(())
    }

    pub async fn place_block(&mut self, pos: BlockPos, block: u16) -> Result<()> {
        let request_id = self.next_request;
        self.next_request += 1;
        let cell = mutate_block(&self.client, request_id, pos, block).await?;
        self.cache.apply_cell(cell.clone());
        self.events.push(RuntimeEvent::MutationApplied { cell });
        Ok(())
    }

    pub async fn remove_block(&mut self, pos: BlockPos) -> Result<()> {
        self.place_block(pos, qivxif_world::AIR).await
    }

    pub async fn apply_command(&mut self, command: ClientCommand) -> Result<()> {
        match command {
            ClientCommand::Place { pos, block } => self.place_block(pos, block).await,
            ClientCommand::Remove { pos } => self.remove_block(pos).await,
        }
    }

    pub fn cache(&self) -> &WorldCache {
        &self.cache
    }

    pub fn summary(&self) -> RuntimeSummary {
        RuntimeSummary {
            world_id: self.world_id.clone(),
            chunks: self.cache.chunk_count(),
            cells: self.cache.cell_count(),
        }
    }

    fn from_joined(client: Client, hello: HelloReceipt, player: &str) -> Self {
        Self {
            client,
            cache: WorldCache::default(),
            events: vec![
                RuntimeEvent::Connected {
                    world_id: hello.world_id.clone(),
                },
                RuntimeEvent::Joined {
                    player: player.to_string(),
                },
            ],
            next_request: 1,
            world_id: hello.world_id,
        }
    }
}
