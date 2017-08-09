use std::io;
use std::fmt::Display;
use std::time::SystemTime;

pub mod sensor_data;
pub mod circular_buffer;
pub mod json_display;
pub mod sensors;
pub mod average;
pub mod historic;

use sensor_data::SensorData;
use circular_buffer::CircularBuffer;
use json_display::JsonDisplay;
use historic::Historic;

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

enum QueuesIndex {
	MINUTE = 0,
	HOUR,
	DAYS,
}

fn main() {
    // array of historic queues of MINUTE, HOUR, DAYS
    let mut historic_queues = [ Historic::<SensorData>::new(32, 24), Historic::<SensorData>::new(128, 120), Historic::<SensorData>::new(9192, 9192)];
    //int sockfd = initSocket(argc >=2 ? argv[1] : NULL);
    println!("Enter loop");
    loop {
		historic_queues[QueuesIndex::MINUTE as usize].add(SensorData::create());
		println!("nbElements (MINUTE) = {}\tnbElements (HOUR) = {}\tnbElements (DAYS) = {}\n",
		historic_queues[QueuesIndex::MINUTE as usize].get_nb_items(),
        historic_queues[QueuesIndex::HOUR as usize].get_nb_items(),
        historic_queues[QueuesIndex::DAYS as usize].get_nb_items()
        );
        Historic::<SensorData>::reduce(& mut historic_queues);
        // treatSocket(sockfd, historicQueues, QUEUE_NBELEMENTS);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_circ_buff() {
        let mut circ_buf = CircularBuffer::<SensorData>::new(5);
        circ_buf.put_item(SensorData::new(SystemTime::now(), 1.0, 11, 12, 13));
        circ_buf.put_item(SensorData::new(SystemTime::now(), 2.0, 21, 22, 23));
        circ_buf.put_item(SensorData::new(SystemTime::now(), 3.0, 31, 32, 33));

        print(&circ_buf);
        
        circ_buf.put_item(SensorData::new(SystemTime::now(), 4.0, 41, 42, 43));
        circ_buf.put_item(SensorData::new(SystemTime::now(), 5.0, 51, 52, 53));
        
        print(&circ_buf);
        
        // Theses variables data5 and data6 should be refused, because the circularbuffer is full
        circ_buf.put_item(SensorData::new(SystemTime::now(), 6.0, 61, 62, 63));
        circ_buf.put_item(SensorData::new(SystemTime::now(), 7.0, 71, 72, 73));
        
        print(&circ_buf);
        
        match circ_buf.get_item() {
            Some(data0bis_unwrap) =>  println!("data0bis = {}", data0bis_unwrap),
            None => println!("No data0bis")
        }

        circ_buf.put_item(SensorData::create());

        print(&circ_buf);
        write_json(&circ_buf, &mut io::stdout());
        
        /*
        assert!(Duration::from_secs(1) != Duration::from_secs(0));
        assert_eq!(Duration::from_secs(1) + Duration::from_secs(2),
                   Duration::from_secs(3));
        assert_eq!(Duration::from_millis(10) + Duration::from_secs(4),
                   Duration::new(4, 10 * 1_000_000));
        assert_eq!(Duration::from_millis(4000), Duration::new(4, 0));
        */
    }
}
