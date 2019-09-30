use super::state::*;
use std::marker::PhantomData;

#[derive(Debug)]
struct StateMachine<S: State> {
    state: PhantomData<S>,
}

impl<S: State> State for StateMachine<S> {
    fn is_final() -> bool {
        S::is_final()
    }
}

edge!(Monday, Tuesday);
edge!(Tuesday, Wednesday);
edge!(Wednesday, Thursday);
edge!(Thursday, Friday);
edge!(Friday, Saturday);
edge!(Saturday, Sunday);
edge!(Sunday, Monday);



