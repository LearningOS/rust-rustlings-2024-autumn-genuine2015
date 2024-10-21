/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    // count: usize, // Useless in Rust.  
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            // count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        // self.count
        self.items.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {

        // Add the value into the end of the vec.
        self.items.push(value);
        // self.count += 1; // This member object has been removed.

        let len = self.len();

        // Use a FnMut Closure to make a series of mutation atomic.
        let mut trinity_repair = |son| -> usize {
            let parent = self.parent_idx(son);

            if parent == 0 {
                return 0;
            }

            if ((self.comparator)(
                self.items.get(son).unwrap(),
                self.items.get(parent).unwrap(),
            )) {
                self.items.swap(son, parent);
            };

            return parent;
        };

        let mut bubble = len;

        while bubble != 0 {
            bubble = trinity_repair(bubble);
        }
        // let mut active;



        // // Repair the heap to keep it legal.
        // while active_parent != 0 {
        //     let parent = active_parent;
        //     let smallest = self.smallest_child_idx(active_parent);

        //     if smallest == 0 { unreachable!() }

        //     if (self.comparator)( 
        //         &self.items[parent],
        //         &self.items[smallest]
        //     ) {
        //         ()
        //     } else {
        //         self.items.swap(parent, smallest)
        //     }

        //     active_parent = self.parent_idx(active_parent)
        // }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.len()
    }

    fn both_children_present(&self, idx: usize) -> bool {
        self.right_child_idx(idx) <= self.len()
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {  
        // left index out of bound.      
        if idx * 2  > self.len() {  
            return usize::MAX;
        }

        // only left index is available. 
        if idx * 2 == self.len() { 
            return idx * 2;
        }
        
        let left  = self.items.get(idx * 2).unwrap();
        let right = self.items.get(idx * 2 + 1).unwrap();
        let smallest;

        if (self.comparator)(left, right) { // left < right 
            smallest = idx * 2 
        } else { // left > right 
            smallest = idx * 2 + 1
        };

        smallest
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut i;
        let last = self.len();

        i = 1usize;

        while i * 2 < last {
            let child  = self.smallest_child_idx(i);
            let ref_child = self.items.get(child).unwrap(); 
            let ref_last  = self.items.get(last ).unwrap();

            if (self.comparator)(ref_child, ref_last) { // ref_child < ref_last
                self.items.swap(i, child);
                i = child;
                continue;
            } else { // ref_child > ref_last
                break;
            }
        }

        self.items.swap(i, last);

        self.items.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}