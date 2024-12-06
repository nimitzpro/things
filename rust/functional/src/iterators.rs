#[derive(Debug)]
pub struct Fibonacci {
    curr: u128,
    next: u128,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u128;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[derive(Debug)]
pub struct SimpleIter {
    values: Vec<i32>,
    index: usize,
}

pub fn make_simple_iter(list: Vec<i32>) -> SimpleIter {
    SimpleIter {
        values: list,
        index: 0,
    }
}

impl Iterator for SimpleIter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.values.len() {
            return None
        }
        
        self.index += 1;
        Some(self.values[self.index - 1])
    }
}

impl SimpleIter {
    pub fn reset(&mut self) -> Option<i32> {
        self.values[self.index] = 0;
        Some(self.values[self.index])
    }
}
