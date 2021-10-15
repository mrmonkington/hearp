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
pub struct Hearp<T: Clone, F: Fn(&T, &T) -> bool> {
    items: Vec<T>,
    comparison_func: F
}

impl<T: Clone, F: Fn(&T, &T) -> bool> Hearp<T, F> {
    pub fn new(comparison_func: F) -> Hearp<T, F> {
        Hearp {
            items: Vec::new(),
            comparison_func
        }
    }

    pub fn insert(&mut self, item: T) {
        self.items.push(item);
        self._upheap(self.items.len() - 1);
    }

    pub fn peek(&self) -> Option<&T> {
        return self.items.get(0);
    }

    fn _upheap(&mut self, index: usize) {
        if index > 0 {
            let pid: usize = _parent_id(index);
            if (self.comparison_func)(
                    &self.items[index],
                    &self.items[pid]
                ) {
                self.items.swap(pid, index);
                self._upheap(pid);
            }
        }
    }

    fn _downheap(&mut self, index: usize) {
        if index < self.items.len() {
            let lid: usize = _left_id(index);
            let rid: usize = _right_id(index);
            let mut favourite: usize = index;

            if lid < self.items.len()
                && (self.comparison_func)(
                    &self.items[lid],
                    &self.items[favourite]
                ) {
                favourite = lid;
            }

            if rid < self.items.len()
                && (self.comparison_func)(
                    &self.items[rid],
                    &self.items[favourite]
                ) {
                favourite = rid;
            }

            // favourite should now contain the parent or one
            // of the two child items
            
            if favourite != index {
                self.items.swap(index, favourite);
                self._downheap(favourite);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // gotta clone it out since we'll be removing it
        let v = self.items.first().cloned();
        match v {
            Some(_) => {
                let last = self.items.pop();
                match last {
                    Some(v_last) if self.items.len() > 0 => {
                        self.items[0] = v_last;
                        self._downheap(0);
                    },
                    _ => {
                        // do nothing, heap is now empty
                    }
                }
            },
            _ => {
                // heap is empty
            }
        }
        return v;
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
        let src: Vec<i32> = (0..100).collect();
        let mut randsrc = src.clone();
        randsrc.shuffle(&mut thread_rng());
        let mut heep = Hearp::new( |x:&i32, y:&i32| x < y );
        for item in randsrc.iter() {
            // don't want to borrow it out of the src list since we want to do a comparison
            // later
            heep.insert(item.clone());
        }
        let mut op: Vec<i32> = Vec::new();
        while let Some(item) = heep.pop() {
            op.push(item);
        }
        assert_eq!(op, src);
    }

    #[test]
    fn test_things() {
        // ranges are an iterator type and need collecting before using as a 
        // contained sequence
      
        // PartialEq asserts that this struct is equal
        #[derive(Debug, Clone, PartialEq)]
        struct Thing {
            value: i32
        }
        
        let src: Vec<Thing> = (0..100).map(|x| Thing{value: x}).collect();
        dbg!(&src);
        let mut randsrc = src.clone();
        randsrc.shuffle(&mut thread_rng());
        let mut heep = Hearp::new( |x:&Thing, y:&Thing| x.value < y.value );
        for item in randsrc.iter() {
            // don't want to borrow it out of the src list since we want to do a comparison
            // later
            heep.insert(item.clone());
        }
        let mut op: Vec<Thing> = Vec::new();
        while let Some(item) = heep.pop() {
            op.push(item);
        }
        assert_eq!(op, src);
    }
}
