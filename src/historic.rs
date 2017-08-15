use std::io;
use average::Average;
use circular_buffer::CircularBuffer;
use std::fmt::Display;
use json_display::JsonDisplay;

pub struct Historic<T> {
    circular_buffer : CircularBuffer<T>,
    limit : usize 
}

impl<T : JsonDisplay + Display> Historic<T> {
    pub fn new(size : usize, limit : usize) -> Historic<T> {
        Historic::<T> {
            circular_buffer : CircularBuffer::<T>::new(size),
            limit
        }
    }
    
    pub fn add(&mut self, element : T) {
        self.circular_buffer.put_item(element);
    }
    
    pub fn get_nb_items(&self) -> usize {
        self.circular_buffer.get_nb_items()
    }
    
    pub fn is_empty(&self) -> bool {
        self.circular_buffer.is_empty()
    }
}

impl<T : JsonDisplay + Display + Average<T>> Historic<T> {
    pub fn reduce(historics : &mut [Historic<T>]) {
        //let mut i = 0;
        let mut average_data = None;
        for historic in historics {
            // look if the previous historic produce an average data to add to the next historic
            match average_data {
                Some(data) => {historic.circular_buffer.put_item(data);}
                None => {},
            }
            if historic.circular_buffer.get_nb_items() > historic.limit {
                let nb_elements_to_sum = historic.limit / 2;
                //println!("Reduction queue {} nbElementsToSum = {}\n", i, nb_elements_to_sum);
                // Accumulate on first nb_elements_to_sum element of the historic
                let mut j = 0;
                let mut accumulator_data = T::empty_cumulator();
                while j < nb_elements_to_sum {
                    match historic.circular_buffer.peek_item(j) {
                            Some(data) => {data.cumulate(&mut accumulator_data);},
                            None => panic!("Problem of implementation in historic reduce method")
                        }
                        j += 1;
                }
                //remove elements used for accumulation from the historic
                j = 0;
                while j < nb_elements_to_sum {
                    historic.circular_buffer.get_item();
                    j += 1;
                }
                // The average_data will be add to the next historic (if historic exists, otherwise it will be lost)
                average_data = Some(T::divide(&accumulator_data, nb_elements_to_sum));
                //i += 1;
            }
            else {
                break;
            }
        }
    }
    
    pub fn write_json_historics(historics : &[Historic<T>], w: &mut io::Write) {
        w.write(b"[").unwrap();
        let mut first = true;
        for historic in historics {
            if !historic.is_empty() {
                if !first {
                    w.write(b",\n").unwrap();
                } else {
                    first = false;
                }
                historic.circular_buffer.write_json_chunk(w).unwrap();
            }
        }
        w.write(b"]\n").unwrap();
    }
}

