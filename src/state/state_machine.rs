use super::state::*;

#[derive(Debug)]
struct StateMachine<S: State> {
    state: S,
}


impl<S: State> State for StateMachine<S> {
    fn is_final(&self) -> bool {
        self.state.is_final()
    }
}

pub enum StateMachineWrapper {
    Monday(StateMachine<Monday>),
    Tuesday(StateMachine<Tuesday>),
    Wednesday(StateMachine<Wednesday>),
    Thursday(StateMachine<Thursday>),
    Friday(StateMachine<Friday>),
    Saturday(StateMachine<Saturday>),
    Sunday(StateMachine<Sunday>),
}

edge!(Monday, Tuesday);
edge!(Tuesday, Wednesday);
edge!(Wednesday, Thursday);
edge!(Thursday, Friday);
edge!(Friday, Saturday);
edge!(Saturday, Sunday);
edge!(Sunday, Monday);

impl StateMachineWrapper {
    pub fn next(self) -> Self {
        match self {
            StateMachineWrapper::Monday(s) => StateMachineWrapper::Tuesday(s.into()),
            StateMachineWrapper::Tuesday(s) => StateMachineWrapper::Wednesday(s.into()),
            StateMachineWrapper::Wednesday(s) => StateMachineWrapper::Thursday(s.into()),
            StateMachineWrapper::Thursday(s) => StateMachineWrapper::Friday(s.into()),
            StateMachineWrapper::Friday(s) => StateMachineWrapper::Saturday(s.into()),
            StateMachineWrapper::Saturday(s) => StateMachineWrapper::Sunday(s.into()),
            StateMachineWrapper::Sunday(s) => StateMachineWrapper::Monday(s.into())
        }
    }
}


