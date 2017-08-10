use std;
use std::fs::File;
use std::io::prelude::*;

fn get<T>(filename : & str) -> T where T : std::str::FromStr, <T>::Err: std::fmt::Display {
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", filename, why),
        Ok(_) => {
            let mut lines = s.lines();
            match lines.next() {
                None => panic!("couldn't extract line {}", s),
                Some(l) => match l.parse::<T>() {
                    Err(why) => panic!("couldn't parse {}: {}", l, why),
                    Ok(val) => val
                }
            }
        }
    }
}

pub fn get_bmp280_pressure() -> f32 {
	get::<f32>("/sys/bus/i2c/devices/i2c-1/1-0076/iio:device1/in_pressure_input")
}

pub fn get_bmp280_temperature() -> i32 {
    get::<i32>("/sys/bus/i2c/devices/i2c-1/1-0076/iio:device1/in_temp_input")
}

pub fn get_htu21_temperature() -> i32{
    get::<i32>("/sys/bus/i2c/devices/i2c-1/1-0040/iio:device0/in_temp_input")
}

pub fn get_htu21_humidity() -> i32{
    get::<i32>("/sys/bus/i2c/devices/i2c-1/1-0040/iio:device0/in_humidityrelative_input")
}
