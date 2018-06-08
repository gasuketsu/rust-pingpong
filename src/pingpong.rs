#[derive(Debug)]
pub enum StateId {
    PING,
    PONG,
}

pub trait State {
    fn current_state(&self) -> StateId;
    fn next_state(self: Box<Self>) -> Box<State>;
}

pub struct PingState;

impl State for PingState {
    fn current_state(&self) -> StateId {
        StateId::PING
    }

    fn next_state(self: Box<Self>) -> Box<State> {
        Box::new(PongState) as Box<State>
    }
}

pub struct PongState;

impl State for PongState {
    fn current_state(&self) -> StateId {
        StateId::PONG
    }

    fn next_state(self: Box<Self>) -> Box<State> {
        Box::new(PingState) as Box<State>
    }
}
