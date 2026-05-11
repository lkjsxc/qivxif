use qivxif_protocol::{RequestId, ServerMsg};
use std::collections::HashMap;

pub struct Session {
    pub id: u64,
    hello: bool,
    joined: bool,
    mutating_responses: HashMap<RequestId, ServerMsg>,
}

impl Session {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            hello: false,
            joined: false,
            mutating_responses: HashMap::new(),
        }
    }

    pub fn mark_hello(&mut self) {
        self.hello = true;
    }

    pub fn mark_joined(&mut self) {
        self.joined = true;
    }

    pub fn can_join(&self) -> bool {
        self.hello
    }

    pub fn can_ping(&self) -> bool {
        self.hello
    }

    pub fn can_play(&self) -> bool {
        self.joined
    }

    pub fn replayed_response(&self, request_id: RequestId) -> Option<ServerMsg> {
        self.mutating_responses.get(&request_id).cloned()
    }

    pub fn remember_response(&mut self, request_id: RequestId, response: &ServerMsg) {
        self.mutating_responses
            .entry(request_id)
            .or_insert_with(|| response.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_order_is_hello_join_play() {
        let mut session = Session::new(1);
        assert!(!session.can_join());
        assert!(!session.can_play());

        session.mark_hello();
        assert!(session.can_join());
        assert!(!session.can_play());

        session.mark_joined();
        assert!(session.can_play());
    }

    #[test]
    fn mutating_responses_are_remembered_once() {
        let mut session = Session::new(1);
        let first = ServerMsg::FlushAck { request_id: 7 };
        let second = ServerMsg::Error {
            code: qivxif_protocol::ErrorCode::FlushError,
            message: "late".to_string(),
        };

        session.remember_response(7, &first);
        session.remember_response(7, &second);

        assert_eq!(session.replayed_response(7), Some(first));
    }
}
