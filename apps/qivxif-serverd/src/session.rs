pub struct Session {
    pub id: u64,
    hello: bool,
    joined: bool,
}

impl Session {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            hello: false,
            joined: false,
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
}
