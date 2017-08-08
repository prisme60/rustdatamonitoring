use std::fmt;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct CircularBuffer<T> {
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
            Some(&self.data[internal_index]);
        }
        None
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


struct CircularBufferIterator<'a, T: 'a> {
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

#[derive(Copy, Clone)]
struct SensorData {
    timestamp: std::time::SystemTime,
    bmp280_pressure: f32,
    bmp280_temperature: i32,
    htu21_temperature: i32,
    htu21_humidity: i32,
}

impl SensorData {
    pub fn new(bmp280_pressure:f32, bmp280_temperature:i32, htu21_temperature:i32, htu21_humidity:i32) -> SensorData {
        SensorData {
            timestamp: SystemTime::now(),
            bmp280_pressure,
            bmp280_temperature,
            htu21_temperature,
            htu21_humidity
        }
    }
}

macro_rules! convTimeMs {
    ($systemtime:expr) => {
        {
            let since_the_epoch = $systemtime.duration_since(UNIX_EPOCH).expect("Time went backwards");
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
        }
    }
}

impl Display for SensorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\ttime      = {}\n\tpressure  = {}\n\tbmp280Temp= {}\n\thtu21Temp = {}\n\thumidity  = {}\n",
            convTimeMs!(self.timestamp),
            self.bmp280_pressure,
            self.bmp280_temperature,
            self.htu21_temperature,
            self.htu21_humidity
        )
    }
}

trait JsonDisplay {
    fn json_item(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl JsonDisplay for SensorData {
    fn json_item(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"timestamp\": {},\n\"pressure\"  : {:.2},\n\"bmp280Temp\": {:.3},\n\"htu21Temp\" : {:.3},\n\"humidity\"  : {:.2}}}\n",
            convTimeMs!(self.timestamp),
            self.bmp280_pressure * 10.0,
            self.bmp280_temperature as f32 / 1000.0,
            self.htu21_temperature as f32 / 1000.0,
            self.htu21_humidity  as f32 / 1000.0
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

fn print<T: Display + JsonDisplay>(cb : &CircularBuffer<T>) {
    println!("=================================================");
    for data in cb {
        println!("{}", data);
    }
}


fn main() {
    let mut circ_buf = CircularBuffer::<SensorData>::new(5);
    circ_buf.put_item(SensorData::new(1.0, 11, 12, 13));
    circ_buf.put_item(SensorData::new(2.0, 21, 22, 23));
    circ_buf.put_item(SensorData::new(3.0, 31, 32, 33));

    print(&circ_buf);
    
    circ_buf.put_item(SensorData::new(4.0, 41, 42, 43));
    circ_buf.put_item(SensorData::new(5.0, 51, 52, 53));
    
    print(&circ_buf);
    
    // Theses variables data5 and data6 should be refused, because the circularbuffer is full
    circ_buf.put_item(SensorData::new(6.0, 61, 62, 63));
    circ_buf.put_item(SensorData::new(7.0, 71, 72, 73));
    
    print(&circ_buf);
    
    match circ_buf.get_item() {
        Some(data0bis_unwrap) =>  println!("data0bis = {}", data0bis_unwrap),
        None => println!("No data0bis")
    }

    print(&circ_buf);
}
