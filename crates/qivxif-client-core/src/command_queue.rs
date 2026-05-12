use qivxif_core::BlockPos;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCommand {
    Place { pos: BlockPos, block: u16 },
    Remove { pos: BlockPos },
}

#[derive(Debug, Default)]
pub struct CommandQueue {
    pending: VecDeque<ClientCommand>,
}

impl CommandQueue {
    pub fn push(&mut self, command: ClientCommand) {
        self.pending.push_back(command);
    }

    pub fn pop(&mut self) -> Option<ClientCommand> {
        self.pending.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
