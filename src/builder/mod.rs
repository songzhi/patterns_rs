#[derive(Default)]
pub struct Foo {
    result: String
}

#[derive(Default)]
pub struct FooBuilder {
    foo: Foo
}

impl FooBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn part1(mut self) -> Self {
        self.foo.result.push_str("part1 ");
        self
    }
    pub fn part2(mut self) -> Self {
        self.foo.result.push_str("part2 ");
        self
    }
    pub fn finish(self) -> Foo {
        self.foo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let foo = FooBuilder::new().part1().part2().finish();
        assert_eq!("part1 part2 ", foo.result.as_str())
    }
}