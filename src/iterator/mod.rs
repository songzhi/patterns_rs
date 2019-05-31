#[derive(Default)]
pub struct Numbers {
    start: isize,
    end: isize,
}

impl Numbers {
    pub fn new(start: isize, end: isize) -> Self {
        Self {
            start,
            end,
        }
    }
    pub fn iter(self) -> NumbersIter {
        NumbersIter {
            next: self.start,
            end: self.end,
        }
    }
}

pub struct NumbersIter {
    end: isize,
    next: isize,
}

impl Iterator for NumbersIter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next <= self.end {
            self.next += 1;
            Some(self.next - 1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = Numbers::new(0, 3);
        let numbers: Vec<_> = numbers.iter().collect();
        assert_eq!(vec![0, 1, 2, 3], numbers);
    }
}