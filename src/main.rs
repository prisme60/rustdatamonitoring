use std::io;
use std::fmt::Display;

pub mod sensor_data;
pub mod circular_buffer;
pub mod json_display;
pub mod sensors;

use sensor_data::SensorData;
use circular_buffer::CircularBuffer;
use json_display::JsonDisplay;

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

fn write_json<T: Display + JsonDisplay>(cb : &CircularBuffer<T>, w: &mut io::Write) {
    let _ = w.write(b"[");
    let _ = cb.write_json_chunk(w);
    let _ = w.write(b"]\n");
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

    circ_buf.put_item(SensorData::create());

    print(&circ_buf);
    write_json(&circ_buf, &mut io::stdout());
}
