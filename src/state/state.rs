pub trait State {
    fn is_final(&self) -> bool;
}

state!(Monday);
state!(Tuesday);
state!(Wednesday);
state!(Thursday);
state!(Friday);
state!(Saturday);
state!(Sunday);
