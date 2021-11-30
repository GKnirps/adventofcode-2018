// a circular, non-empty double linked list (in a way)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CircularList<T> {
    data: Vec<CircularListElement<T>>,
    current: usize,
}

impl<T> CircularList<T> {
    pub fn with_capacity(capacity: usize, first_element: T) -> CircularList<T> {
        let mut elements = Vec::with_capacity(capacity);
        elements.push(CircularListElement {
            payload: first_element,
            left_index: 0,
            right_index: 0,
        });
        return CircularList {
            data: elements,
            current: 0,
        };
    }

    pub fn move_right(&mut self) {
        self.current = self.data[self.current].right_index;
    }

    pub fn move_right_n(&mut self, n: usize) {
        for _ in 0..n {
            self.move_right();
        }
    }

    pub fn move_left(&mut self) {
        self.current = self.data[self.current].left_index;
    }

    pub fn move_left_n(&mut self, n: usize) {
        for _ in 0..n {
            self.move_left();
        }
    }

    pub fn insert_right(&mut self, value: T) {
        let left_index = self.current;
        let right_index = self.data[self.current].right_index;
        let new_index = self.data.len();
        self.data.push(CircularListElement {
            payload: value,
            left_index,
            right_index,
        });
        self.data[right_index].left_index = new_index;
        self.data[self.current].right_index = new_index;
    }

    // remove the current element and use the element to the left as new current element
    // IMPORTANT: this does not really remove the value (frees space) but only removes its
    // place in the circle
    // returns a borrow of the removed value
    // also, cannot remove the last element and will panic if trying to
    pub fn remove_use_right(&mut self) -> &T {
        let right_index = self.data[self.current].right_index;
        let left_index = self.data[self.current].left_index;
        let old_index = self.current;
        if right_index == old_index {
            panic!("Cannot remove last element!");
        }
        self.data[right_index].left_index = left_index;
        self.data[left_index].right_index = right_index;
        self.current = right_index;
        return &self.data[old_index].payload;
    }

    pub fn current_value(&self) -> &T {
        &self.data[self.current].payload
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CircularListElement<T> {
    payload: T,
    left_index: usize,
    right_index: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_list_elements_work_correctly() {
        // given
        let mut circle: CircularList<u32> = CircularList::with_capacity(10, 42);

        // when/then
        assert_eq!(circle.current_value(), &42);
        circle.move_right();
        assert_eq!(circle.current_value(), &42);
        circle.move_left();
        assert_eq!(circle.current_value(), &42);
    }

    #[test]
    fn insert_right_works_correctly() {
        // given
        let mut circle: CircularList<u32> = CircularList::with_capacity(10, 42);

        // when
        circle.insert_right(9001);
        circle.insert_right(1337);

        // then
        assert_eq!(circle.current_value(), &42);
        circle.move_right();
        assert_eq!(circle.current_value(), &1337);
        circle.move_right();
        assert_eq!(circle.current_value(), &9001);
        circle.move_right();
        assert_eq!(circle.current_value(), &42);
    }

    #[test]
    fn remove_use_right_works_correctly() {
        // given
        let mut circle: CircularList<u32> = CircularList::with_capacity(10, 42);
        circle.insert_right(9001);
        circle.insert_right(1337);

        // when
        circle.remove_use_right();

        // then
        assert_eq!(circle.current_value(), &1337);
        circle.move_left();
        assert_eq!(circle.current_value(), &9001);
        circle.move_left();
        assert_eq!(circle.current_value(), &1337);
    }

    #[test]
    fn move_right_n_works_correctly() {
        // given
        let mut circle: CircularList<u32> = CircularList::with_capacity(10, 42);
        circle.insert_right(9001);
        circle.insert_right(1337);
        circle.insert_right(313);

        // when
        circle.move_right_n(2);
        assert_eq!(circle.current_value(), &1337);
    }

    #[test]
    fn move_left_n_works_correctly() {
        // given
        let mut circle: CircularList<u32> = CircularList::with_capacity(10, 42);
        circle.insert_right(9001);
        circle.insert_right(1337);
        circle.insert_right(313);

        // when
        circle.move_left_n(2);

        // then
        assert_eq!(circle.current_value(), &1337);
    }
}
