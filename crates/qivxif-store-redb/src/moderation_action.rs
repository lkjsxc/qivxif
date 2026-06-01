use crate::moderation::ModerationAction;
use qivxif_graph::EdgeKind;
use qivxif_history::OperationKind;

impl ModerationAction {
    pub(crate) fn edge_kind(self) -> EdgeKind {
        match self {
            Self::Mute => EdgeKind::Mutes,
            Self::Block => EdgeKind::Blocks,
        }
    }

    pub(crate) fn create_op(self) -> OperationKind {
        match self {
            Self::Mute => OperationKind::SocialMute,
            Self::Block => OperationKind::SocialBlock,
        }
    }

    pub(crate) fn clear_op(self) -> OperationKind {
        match self {
            Self::Mute => OperationKind::SocialUnmute,
            Self::Block => OperationKind::SocialUnblock,
        }
    }

    pub(crate) fn state_name(self) -> &'static str {
        match self {
            Self::Mute => "mute",
            Self::Block => "block",
        }
    }

    pub(crate) fn clear_reason(self) -> &'static str {
        match self {
            Self::Mute => "unmute",
            Self::Block => "unblock",
        }
    }
}
