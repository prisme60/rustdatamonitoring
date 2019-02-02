use std::{
    fmt::{self, Display},
    io,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{average::Average, json_display::JsonDisplay, sensors::Sensor};

macro_rules! convTimeEpochDuration {
    ($systemtime:expr) => {
        $systemtime
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
    };
}

macro_rules! convDurationMs {
    ($duration:expr) => {
        $duration.as_secs() * 1000 + $duration.subsec_nanos() as u64 / 1_000_000
    };
}

#[derive(Copy, Clone)]
pub struct SensorData {
    timestamp: Duration,
    bmp280_pressure: f32,
    bmp280_temperature: i32,
    htu21_temperature: i32,
    htu21_humidity: i32,
}

pub struct SensorCumulatedData {
    timestamp: Duration,
    bmp280_pressure: f64,
    bmp280_temperature: i64,
    htu21_temperature: i64,
    htu21_humidity: i64,
}

impl SensorData {
    pub fn new(
        timestamp: SystemTime,
        bmp280_pressure: f32,
        bmp280_temperature: i32,
        htu21_temperature: i32,
        htu21_humidity: i32,
    ) -> SensorData {
        SensorData {
            timestamp: convTimeEpochDuration!(timestamp),
            bmp280_pressure,
            bmp280_temperature,
            htu21_temperature,
            htu21_humidity,
        }
    }

    pub fn create(
        sensor_bmp280_pressure: &Sensor,
        sensor_bmp280_temperature: &Sensor,
        sensor_htu21_temperature: &Sensor,
        sensor_htu21_humidity: &Sensor,
    ) -> SensorData {
        SensorData::new(
            SystemTime::now(),
            sensor_bmp280_pressure.get::<f32>(),
            sensor_bmp280_temperature.get::<i32>(),
            sensor_htu21_temperature.get::<i32>(),
            sensor_htu21_humidity.get::<i32>(),
        )
    }

    pub fn get_bmp280_pressure(&self) -> f32 {
        self.bmp280_pressure
    }

    pub fn get_bmp280_temperature(&self) -> i32 {
        self.bmp280_temperature
    }

    pub fn get_htu21_temperature(&self) -> i32 {
        self.htu21_temperature
    }

    pub fn get_htu21_humidity(&self) -> i32 {
        self.htu21_humidity
    }
}

impl Display for SensorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\ttime      = {}\n\tpressure  = {}\n\tbmp280Temp= {}\n\thtu21Temp = {}\n\thumidity  = {}\n",
            convDurationMs!(self.timestamp),
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
            convDurationMs!(self.timestamp),
            self.bmp280_pressure * 10.0,
            self.bmp280_temperature as f32 / 1000.0,
            self.htu21_temperature as f32 / 1000.0,
            self.htu21_humidity  as f32 / 1000.0
        ))
    }
}

impl Average<SensorData> for SensorData {
    type Acc = SensorCumulatedData;

    fn empty_cumulator() -> Self::Acc {
        SensorCumulatedData {
            timestamp: Duration::new(0, 0),
            bmp280_pressure: 0.0,
            bmp280_temperature: 0,
            htu21_temperature: 0,
            htu21_humidity: 0,
        }
    }

    fn cumulate<'a, 'b>(&'a self, cumulated_data: &'b mut Self::Acc) -> &'b Self::Acc {
        cumulated_data.timestamp += self.timestamp;
        cumulated_data.bmp280_pressure += self.bmp280_pressure as f64;
        cumulated_data.bmp280_temperature += self.bmp280_temperature as i64;
        cumulated_data.htu21_temperature += self.htu21_temperature as i64;
        cumulated_data.htu21_humidity += self.htu21_humidity as i64;
        cumulated_data
    }

    fn divide(cumulated_data: &Self::Acc, nb_elements: usize) -> SensorData {
        SensorData {
            timestamp: (cumulated_data.timestamp / nb_elements as u32),
            bmp280_pressure: (cumulated_data.bmp280_pressure / nb_elements as f64) as f32,
            bmp280_temperature: (cumulated_data.bmp280_temperature / nb_elements as i64) as i32,
            htu21_temperature: (cumulated_data.htu21_temperature / nb_elements as i64) as i32,
            htu21_humidity: (cumulated_data.htu21_humidity / nb_elements as i64) as i32,
        }
    }
}
