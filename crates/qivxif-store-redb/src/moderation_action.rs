use crate::moderation::ModerationAction;
use qivxif_graph::EdgeKind;
use qivxif_history::EventKind;

impl ModerationAction {
    pub(crate) fn edge_kind(self) -> EdgeKind {
        match self {
            Self::Mute => EdgeKind::Mutes,
            Self::Block => EdgeKind::Blocks,
        }
    }

    pub(crate) fn create_event(self) -> EventKind {
        match self {
            Self::Mute => EventKind::SocialMute,
            Self::Block => EventKind::SocialBlock,
        }
    }

    pub(crate) fn clear_event(self) -> EventKind {
        match self {
            Self::Mute => EventKind::SocialUnmute,
            Self::Block => EventKind::SocialUnblock,
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
