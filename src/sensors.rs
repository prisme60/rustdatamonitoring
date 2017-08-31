use std;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;


pub struct Sensor {
    filename : String,
}

impl Sensor {
    /// Probe permits to find if index of the sensor for iio:device in the sys fs
    ///
    /// The supplied path shall contain a "{}" in order to indicate where the index have to be inserted
    ///
    pub fn probe(path : &str) -> Result<Sensor, String> {
        for i in 0..10 {
            let path_to_test = path.replace("{}", i.to_string().as_str());
            if Path::new(&path_to_test).exists() {
                return Ok(Sensor{filename : String::from(path_to_test)});
            }
        }
        Err("Can't find the driver in the sysfs : ".to_string() + path)
    }

    pub fn get<T>(&self) -> T where T : std::str::FromStr, <T>::Err: std::fmt::Display {
        get::<T>(self.filename.as_str())
    }
}


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
/*
// Old way (not always working, because index of iio:device was not adapted)

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
*/