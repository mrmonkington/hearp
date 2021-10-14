fn _parent_id(index: usize) -> usize {
    (index - 1).div_euclid(2)
}

fn _left_id(index: usize) -> usize {
    index * 2 + 1
}

fn _right_id(index: usize) -> usize {
    index * 2 + 2
}

// #[derive(Clone, Copy)]
struct Hearp<T> {
    items: Vec<T>,
    comparison_func: fn(&T, &T) -> bool
}

impl<T: Clone> Hearp<T> {
    pub fn insert(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn peek(&self) -> Option<&T> {
        return self.items.get(0);
    }

    fn _downheap(&mut self, index: usize) {
        if index < self.items.len() {
            let lid: usize = _left_id(index);
            let rid: usize = _right_id(index);
            let favourite: usize = index;

            if lid < self.items.len()
                && (self.comparison_func)(
                    &self.items[lid],
                    &self.items[favourite]
                ) {
                favourite = lid;
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        // gotta clone it out since we'll be removing it
        let v = self.items.first().cloned();
        match v {
            Some(_) => {
                let last = self.items.pop();
                match last {
                    Some(v_last) => {
                        self.items.insert(0, v_last);
                        self._downheap(0);
                    },
                    None => {
                        // do nothing, heap is now empty
                    }
                }
            },
            None => {
                // heap is empty
            }
        }
        return v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;


    #[test]
    fn parent_id() {
        assert_eq!(_parent_id(1), 0);
        assert_eq!(_parent_id(2), 0);
        assert_eq!(_parent_id(3), 1);
        assert_eq!(_parent_id(4), 1);
        assert_eq!(_parent_id(5), 2);
        assert_eq!(_parent_id(6), 2);
    }

    #[test]
    fn left_id() {
        assert_eq!(_left_id(0), 1);
        assert_eq!(_left_id(1), 3);
    }

    #[test]
    fn right_id() {
        assert_eq!(_right_id(0), 2);
        assert_eq!(_right_id(1), 4);
    }

    #[test]
    fn test_ints() {
        // ranges are an iterator type and need collecting before using as a 
        // contained sequence
        let mut src: Vec<i32> = (0..100).collect();
        src.shuffle(&mut thread_rng());
        let mut heep: Hearp<i32> = Hearp {
            items: Vec::new(),
            comparison_func: |x:&i32, y:&i32| x < y
        };
        for item in src.iter() {
            heep.insert(*item);
        }
    }
}