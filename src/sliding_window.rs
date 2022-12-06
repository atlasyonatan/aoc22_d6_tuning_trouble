///Uses an fixed size vec and keeps track of order of insertions.
///Once it is full the next push will replace the oldest item.
pub struct SlidingWindow<Item> {
    vec: Vec<Item>,
    insert_index: usize,
}

impl<Item> SlidingWindow<Item> {
    pub fn with_capacity(capacity: usize) -> SlidingWindow<Item> {
        SlidingWindow {
            vec: Vec::with_capacity(capacity),
            insert_index: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    ///Gets the item at index by order of insertions ascending.
    pub fn get(&self, index: usize) -> Option<&Item> {
        if index >= self.vec.capacity() {
            return None;
        }
        return self.vec.get((self.insert_index + index) % self.capacity());
    }

    pub fn push(&mut self, value: Item) {
        if self.vec.len() < self.capacity() {
            self.vec.push(value)
        } else {
            self.vec[self.insert_index] = value;
            self.insert_index = (self.insert_index + 1) % self.capacity();
        }
    }

    ///Iterates over the items of the window by order of insertions ascending.
    #[allow(dead_code)]
    pub fn iter<'a>(&'a self) -> OrderedIter<'a, Item> {
        let mut sli = OrderedIter {
            sw: self,
            end: self.vec.len() - 1,
            next: 0,
            stop: false,
        };
        if self.vec.len() == self.capacity() {
            sli.next = self.insert_index;
            sli.end = (self.insert_index - 1) % self.capacity();
        }
        sli
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.vec
    }
}

pub struct OrderedIter<'a, Item> {
    sw: &'a SlidingWindow<Item>,
    end: usize,
    next: usize,
    stop: bool,
}

impl<'a, Item> Iterator for OrderedIter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }
        if self.next == self.end {
            self.stop = true;
        }
        let value = &self.sw.vec[self.next];
        self.next = (self.next + 1) % self.sw.capacity();
        return Some(value);
    }
}

//tried to extend Iterator to work with SlidingWindow but can't both keep a mutable state and yield a borrow of that state by Iterator api

// pub struct Adapter<I: Iterator> {
//     iter: I,
//     sw: SlidingWindow<<I as Iterator>::Item>,
// }

// impl<I: Iterator> Adapter<I> {
//     fn sw<'a>(&'a self) -> &SlidingWindow<<I as Iterator>::Item> {
//         &self.sw
//     }
// }

// impl<I: Iterator, Item:Copy> Iterator for Adapter<I>
// where I<Item = Item>
// {
//     type Item = Vec<<I as Iterator>::Item>;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             match self.iter.next() {
//                 Some(item) => {
//                     self.sw.push(item);
//                 }
//                 None => return None,
//             }
//             if self.sw.is_full() {
//                 break;
//             }
//         }
//         Some(self.sw.iter().collect())
//     }
// }

// pub trait Windows: Iterator + Sized {
//     fn windows<'a>(self, size: usize) -> Adapter<Self>;
// }

// impl<I: Iterator> Windows for I {
//     fn windows<'a>(self, size: usize) -> Adapter<Self> {
//         Adapter {
//             iter: self,
//             sw: SlidingWindow::with_capacity(size),
//         };
//     }
// }
