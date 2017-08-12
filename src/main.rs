use std::time::{Instant, Duration};
use std::env;

#[cfg(test)]
use std::fmt::Display;
#[cfg(test)]
use std::io;
#[cfg(test)]
use std::time::SystemTime;

pub mod sensor_data;
pub mod circular_buffer;
pub mod json_display;
pub mod sensors;
pub mod average;
pub mod historic;
pub mod server;

use sensor_data::SensorData;
use historic::Historic;
use server::Server;

#[cfg(test)]
use circular_buffer::CircularBuffer;
#[cfg(test)]
use json_display::JsonDisplay;

#[allow(dead_code)]
enum QueuesIndex {
	MINUTE = 0,
	HOUR,
	DAYS,
}

const DEFAULT_SOCKET_NAME : &str = "rustSocket";
// Sampling time in milliceconds
static SAMPLING_TIME_MS : u64 = 5000;


fn main() { 
    let socket_name = match env::args().nth(1) {
        Some(arg) => arg,
        None => DEFAULT_SOCKET_NAME.to_string()
    };
    println!("Socket name : {}", socket_name);
    
    let sampling_duration_ms = Duration::from_millis(SAMPLING_TIME_MS);
    // array of historic queues of MINUTE, HOUR, DAYS
    let mut historic_queues = [ Historic::<SensorData>::new(32, 24), Historic::<SensorData>::new(128, 120), Historic::<SensorData>::new(9192, 9192)];
    let (_, rx) = Server::create_server_thread(socket_name.as_str());
    //println!("Enter loop");
    loop {
		historic_queues[QueuesIndex::MINUTE as usize].add(SensorData::create());
		//println!("nbElements (MINUTE) = {}\tnbElements (HOUR) = {}\tnbElements (DAYS) = {}\n",
		//historic_queues[QueuesIndex::MINUTE as usize].get_nb_items(),
        //historic_queues[QueuesIndex::HOUR as usize].get_nb_items(),
        //historic_queues[QueuesIndex::DAYS as usize].get_nb_items()
        //);
        Historic::<SensorData>::reduce(& mut historic_queues);
        
        // treatSocket(sockfd, historicQueues, QUEUE_NBELEMENTS);
        let now = Instant::now();
        while now.elapsed() <= sampling_duration_ms {
            match rx.recv_timeout(sampling_duration_ms) {
                Err(_/*err*/) => () /*println!("no request {}", err) */,
                Ok(mut stream) => {
                    Historic::<SensorData>::write_json_historics(& historic_queues, &mut stream);
                }
            }
        }
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

#[cfg(test)]
fn print<T: Display + JsonDisplay>(cb : &CircularBuffer<T>) {
    println!("=================================================");
    for data in cb {
        println!("{}", data);
    }
}

#[cfg(test)]
fn write_json<T: Display + JsonDisplay>(cb : &CircularBuffer<T>, w: &mut io::Write) {
    let _ = w.write(b"[");
    let _ = cb.write_json_chunk(w);
    let _ = w.write(b"]\n");
}
 
#[test]
fn test_circ_buff() {
    let mut circ_buf = CircularBuffer::<SensorData>::new(5);
    circ_buf.put_item(SensorData::new(SystemTime::now(), 1.0, 11, 12, 13));
    circ_buf.put_item(SensorData::new(SystemTime::now(), 2.0, 21, 22, 23));
    circ_buf.put_item(SensorData::new(SystemTime::now(), 3.0, 31, 32, 33));
    
    assert_eq!(circ_buf.get_nb_items(),3);

    print(&circ_buf);
    
    circ_buf.put_item(SensorData::new(SystemTime::now(), 4.0, 41, 42, 43));
    circ_buf.put_item(SensorData::new(SystemTime::now(), 5.0, 51, 52, 53));
    
    assert_eq!(circ_buf.get_nb_items(),5);
    
    print(&circ_buf);
    
    // Theses variables data5 and data6 should be refused, because the circularbuffer is full
    circ_buf.put_item(SensorData::new(SystemTime::now(), 6.0, 61, 62, 63));
    circ_buf.put_item(SensorData::new(SystemTime::now(), 7.0, 71, 72, 73));
    
    assert_eq!(circ_buf.get_nb_items(),5);
    
    print(&circ_buf);
    
    match circ_buf.get_item() {
        Some(data0bis_unwrap) =>  {
            println!("data0bis = {}", data0bis_unwrap);
            assert_eq!(data0bis_unwrap.get_bmp280_pressure(),1.0);
            assert_eq!(data0bis_unwrap.get_bmp280_temperature(),11);
            assert_eq!(data0bis_unwrap.get_htu21_temperature(),12);
            assert_eq!(data0bis_unwrap.get_htu21_humidity(),13);    
        },
        None => println!("No data0bis")
    }

    // This method will fail without the i2c devices BMP280 and hut21 on the system
    // circ_buf.put_item(SensorData::create());
    // print(&circ_buf);
    
    write_json(&circ_buf, &mut io::stdout());
}
