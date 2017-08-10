use std::io;
use std::fmt::Display;
use json_display::JsonDisplay;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    first: usize,
    last: usize,
    valid_items: usize,
    data: Vec<T>,
    max_items: usize,
}

impl<T: Display + JsonDisplay> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            first: 0,
            last: 0,
            valid_items: 0,
            data: Vec::<T>::with_capacity(size),
            max_items: size,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.valid_items == 0
    }

    pub fn get_nb_items(&self) -> usize {
        self.valid_items
    }

    pub fn put_item(&mut self, item_value: T) -> bool {
        if self.valid_items >= self.max_items {
            println!("The queue is full\n");
            println!("You cannot add items\n");
            false
        } else {
            self.valid_items += 1;
            if self.last + 1 < self.data.len() {
                self.data[self.last as usize] = item_value;
            } else {
                self.data.push(item_value);
            }
            self.last = (self.last + 1) % self.max_items;
            true
        }
    }

    pub fn get_item(&mut self) -> Option<&T> {
        if self.is_empty() {
            println!("isempty\n");
            None
        } else {
            let index = self.first;
            self.first = (self.first + 1) % self.max_items;
            self.valid_items -= 1;
            Some(&self.data[index])
        }
    }

    pub fn peek_item(&self, index: usize) -> Option<&T> {
        if index < self.valid_items {
            let internal_index = (self.first + index) % self.max_items;
            Some(&self.data[internal_index])
        }
        else {
            None
        }
    }
    
    pub fn write_json_chunk(&self, w: &mut io::Write) -> io::Result<()> {
        let mut first = true;
        let mut result : io::Result<()> = Ok(());
        for data in self {
            match result {
                Ok(_) => {
                    if !first {
                        let _ = w.write(b",");
                    } else {
                        first = false;
                    }
                    result = data.json_item(w);
                },
                Err(_) => { println!("error writing json"); }
            }
        }
        result
    }
}

// IntToIterator is fully functionnal
impl<'a, T:'a + Display + JsonDisplay> IntoIterator for &'a CircularBuffer<T> {
    type Item = &'a T;
    type IntoIter = CircularBufferIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CircularBufferIterator {
            circular_buffer : self,
            current_index : self.first,
            remaining_items : self.get_nb_items()
        }
    }
}


pub struct CircularBufferIterator<'a, T: 'a> {
    circular_buffer: &'a CircularBuffer<T>,
    current_index: usize,
    remaining_items: usize,
}

/* // This constructor is no more need, since inToIterator compile
impl<'a, T: 'a + Display> CircularBufferIterator<'a, T> {
    fn new(circular_buffer : &'a CircularBuffer<T>) -> CircularBufferIterator<T> {
         CircularBufferIterator {
            circular_buffer : circular_buffer,
            current_index : circular_buffer.first,
            remaining_items : circular_buffer.get_nb_items()
        }
    }
}
*/

impl<'a, T> Iterator for CircularBufferIterator<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<&'a T> {
        if self.remaining_items > 0 {
            let current_index_before_update = self.current_index;
            self.current_index = (self.current_index + 1) % self.circular_buffer.max_items;
            self.remaining_items -= 1;
            Some(&self.circular_buffer.data[current_index_before_update])
        }
        else {
            None
        }
    }
}
