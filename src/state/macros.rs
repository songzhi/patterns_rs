macro_rules! state {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name;
        impl State for $name {
            fn is_final() -> bool {
                false
            }
        }
    };
}

macro_rules! edge {
    ($from:ty, $to:ident) => {
        impl From<StateMachine<$from>> for StateMachine<$to> {
            fn from(st: StateMachine<$from>) -> Self {
                unsafe {
                    // 这里是安全的,因为我们用了零字节大小的PhantomData<S>
                    std::mem::transmute::<StateMachine<$from>,StateMachine<$to>>(st)
                }
            }
        }
    };
}
