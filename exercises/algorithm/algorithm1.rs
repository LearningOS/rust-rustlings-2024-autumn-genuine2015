/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::collections::linked_list;
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    /// To create an empty linked list.
    /// 
    /// # Example: 
    /// 
    /// ```
    /// fn create_empty_list() {
    ///     let mut list = LinkedList::<String>::new();
    ///
    ///     assert_eq!(0, list.length);
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    /// To add a new object at the end of your linked list.
    /// 
    /// # Example: 
    /// 
    /// ```
    /// fn create_numeric_list() {
    ///     let mut list = LinkedList::<i32>::new();
    /// 
    ///     list.add(1);
    ///     list.add(2);
    ///     list.add(3);
    ///     println!("Linked List is {}", list);
    ///     assert_eq!(3, list.length);
    /// }
    /// ```
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        unsafe {
            let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.end {
                None => self.start = node_ptr,
                Some(end_ptr) => (*end_ptr.as_ptr()).next = node_ptr,
            }
            self.end = node_ptr;
            self.length += 1;
        }
    }

    /// To get the reference of the index'th value.
    /// 
    /// # Example: 
    /// 
    /// ```
    /// fn create_numeric_list() {
    ///     let mut list = LinkedList::<i32>::new();
    /// 
    ///     list.add(1);
    ///     list.add(2);
    ///     list.add(3);
    ///     println!("Linked List is {}", list);
    ///     assert_eq!(3, list.length);
    /// 
    ///     assert_eq!(list.get(0), Option(&1));
    ///     assert_eq!(list.get(1), Option(&2));
    ///     assert_eq!(list.get(2), Option(&3));
    /// }
    /// ```
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /// To take away the first node, and return its value.
    /// 
    /// # Example: 
    /// 
    /// ```
    /// fn test_sub() {
    ///     let mut list = LinkedList::<i32>::new();
    ///     list.add(1);
    ///     list.add(2);
    ///     list.add(3);
    ///     println!("Linked List is {}", list);
    ///     assert_eq!(3, list.length);
    ///
    ///     assert_eq!(list.sub(), Some(1));
    ///     assert_eq!(2, list.length);
    ///
    ///     assert_eq!(list.sub(), Some(2));
    ///     assert_eq!(1, list.length);
    ///
    ///     assert_eq!(list.sub(), Some(3));
    ///     assert_eq!(0, list.length);
    ///
    ///     assert_eq!(list.sub(), None);
    ///     assert_eq!(0, list.length);
    /// }
    /// ```
    pub fn take(&mut self) -> Option<T> {
        let start_ptr = self.start?; // if self.start == None, return it.

        let val = unsafe {
            let start_box = Box::from_raw(start_ptr.as_ptr());

            self.start = start_box.next;
            self.length -= 1;

            start_box.val
        };

        Some(val)
    }
    
    // pub fn insert(&mut self, index: i32, obj: T) {
    //     self.insert_ith_node(self.start, index, obj);
    // }

    // fn insert_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32, obj: T) {
    //     match node {
    //         None => None,
    //         Some(next_ptr) => match index {
    //             0 => self.insert_node(node, obj),
    //             _ => self.insert_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1, obj),
    //         },
    //     }
    // }

    // fn insert_node(&mut self, prev_ptr: Option<NonNull<Node<T>>>, obj: T) -> Result<(), ()>{
    //     if prev_ptr.is_none() {
    //         return Err(());
    //     }

    //     let mut new_box = Box::new(Node::new(obj));

    //     unsafe {
    //         // SAFETY: Because of the guard code at the beginning, 
    //         // this ptr will now be always some.
    //         let prev_mut = prev_ptr.unwrap_unchecked().as_mut();
    //         new_box.next = prev_mut.next;

    //         let new_ptr = Some(NonNull::new_unchecked(Box::into_raw(new_box)));
    //         prev_mut.next = new_ptr;

    //         self.length += 1;
    //     }

    //     Ok(())
    // }

    

}

impl<T: Ord> LinkedList<T> {
    // fn merge_insert(&mut self, obj: T) -> Result<(),()> {
    //     let mut ptr = self.start;

    //     if ptr.is_none() {
    //         return Err(());
    //     }

    //     while let Some(nonnull) = ptr {
    //         if &obj < unsafe { &nonnull.as_ref().val } {
    //             break;
    //         } else {
    //             ptr = unsafe { nonnull.as_ref() }.next;
    //             continue;
    //         };
    //     }
    //     self.insert_node(ptr, obj);

    //     Ok(())
    // }

	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
	{  
        let mut list_res = LinkedList::<T>::new();
        let mut buf_a: Option<T> = None;
        let mut buf_b: Option<T> = None;

        loop {
            buf_a = buf_a.or_else(|| list_a.take());
            buf_b = buf_b.or_else(|| list_b.take());
            
            let next_obj = match (&buf_a, &buf_b) {
                (Some(_)    , None)                  => buf_a.take().unwrap(),
                (None       , Some(_))               => buf_b.take().unwrap(), 
                (Some(a), Some(b)) if a < b  => buf_a.take().unwrap(),
                (Some(a), Some(b))           => buf_b.take().unwrap(),
                (None       , None)                  => break,
            };

            list_res.add(next_obj);
        };

        list_res
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_take() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);

        assert_eq!(list.take(), Some(1));
        assert_eq!(2, list.length);

        assert_eq!(list.take(), Some(2));
        assert_eq!(1, list.length);

        assert_eq!(list.take(), Some(3));
        assert_eq!(0, list.length);

        assert_eq!(list.take(), None);
        assert_eq!(0, list.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}