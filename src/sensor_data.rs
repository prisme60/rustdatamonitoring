use std::io;
use std::fmt;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

use json_display::JsonDisplay;
use sensors::*;

#[derive(Copy, Clone)]
pub struct SensorData {
    timestamp: SystemTime,
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
    
    pub fn create() -> SensorData {
       	SensorData::new(get_pressure(), get_bmp280_temperature(), get_htu21_temperature(), get_htu21_humidity())
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

impl JsonDisplay for SensorData {
    fn json_item(&self, w: &mut io::Write) -> io::Result<()> {
        w.write_fmt(format_args!(
            "{{\"timestamp\": {},\n\"pressure\"  : {:.2},\n\"bmp280Temp\": {:.3},\n\"htu21Temp\" : {:.3},\n\"humidity\"  : {:.2}}}\n",
            convTimeMs!(self.timestamp),
            self.bmp280_pressure * 10.0,
            self.bmp280_temperature as f32 / 1000.0,
            self.htu21_temperature as f32 / 1000.0,
            self.htu21_humidity  as f32 / 1000.0
        ))
    }
}
