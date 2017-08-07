use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct CircularBuffer<T> {
    first: usize,
    last: usize,
    valid_items: usize,
    data: Vec<T>,
    max_items: usize,
}

impl<T: Display> CircularBuffer<T> {
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
            Some(&self.data[internal_index]);
        }
        None
    }
}

// IntToIterator is fully functionnal
impl<'a, T:'a + Display> IntoIterator for &'a CircularBuffer<T> {
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


struct CircularBufferIterator<'a, T: 'a> {
    circular_buffer: &'a CircularBuffer<T>,
    current_index: usize,
    remaining_items: usize,
}

/* // This constructor is no more nedd, since inToIterator compile
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

#[derive(Copy, Clone)]
struct SensorData {
    /*timestamp: std::time::Instant,*/
    bmp280_pressure: f32,
    bmp280_temperature: i32,
    htu21_temperature: i32,
    htu21_humidity: i32,
}

impl Display for SensorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\tpressure  = {}\n\tbmp280Temp= {}\n\thtu21Temp = {}\n\thumidity  = {}\n",
            self.bmp280_pressure,
            self.bmp280_temperature,
            self.htu21_temperature,
            self.htu21_humidity
        )
    }
}

/* // The expected way I want to write my for loop iteration (no more need to call an explicit constructor)
 * // but currently, IntoIterator didn't compile
fn print<T: Display>(cb : &CircularBuffer<T>) {
    println!("=================================================");
    for data in CircularBufferIterator::new(&cb) {
        println!("{}", data);
    }
}
*/

fn print<T: Display>(cb : &CircularBuffer<T>) {
    println!("=================================================");
    for data in cb {
        println!("{}", data);
    }
}


fn main() {
    let mut circ_buf = CircularBuffer::<SensorData>::new(5);
    let data0 = SensorData {
        bmp280_pressure: 1.0,
        bmp280_temperature: 11,
        htu21_temperature: 12,
        htu21_humidity: 13,
    };
    circ_buf.put_item(data0);
    let data1 = SensorData {
        bmp280_pressure: 2.0,
        bmp280_temperature: 21,
        htu21_temperature: 22,
        htu21_humidity: 23,
    };
    circ_buf.put_item(data1);
    let data2 = SensorData {
        bmp280_pressure: 3.0,
        bmp280_temperature: 31,
        htu21_temperature: 32,
        htu21_humidity: 33,
    };
    circ_buf.put_item(data2);
    
    print(&circ_buf);
    
    let data3 = SensorData {
        bmp280_pressure: 4.0,
        bmp280_temperature: 41,
        htu21_temperature: 42,
        htu21_humidity: 43,
    };
    circ_buf.put_item(data3);
    
    let data4 = SensorData {
        bmp280_pressure: 53.0,
        bmp280_temperature: 51,
        htu21_temperature: 52,
        htu21_humidity: 53,
    };
    circ_buf.put_item(data4);
    
    print(&circ_buf);
    
    // Theses variables data5 and data6 should be refused, because the circularbuffer is full
    let data5 = SensorData {
        bmp280_pressure: 6.0,
        bmp280_temperature: 61,
        htu21_temperature: 62,
        htu21_humidity: 63,
    };
    circ_buf.put_item(data5);
    
    let data6 = SensorData {
        bmp280_pressure: 73.0,
        bmp280_temperature: 71,
        htu21_temperature: 72,
        htu21_humidity: 73,
    };
    circ_buf.put_item(data6);
    
    print(&circ_buf);
    
    match circ_buf.get_item() {
        Some(data0bis_unwrap) =>  println!("data0bis = {}", data0bis_unwrap),
        None => println!("No data0bis")
    }

    print(&circ_buf);
}
