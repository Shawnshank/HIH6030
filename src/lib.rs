#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// HIH6030 driver
pub struct hih6030<I2C> {
    i2c: I2C,
    address: u8,
}

pub enum Status {
    Ok,
    StaleData,
    CommandMode,
    Unknown,
}

pub struct Measurement {
    pub status: Status,
    pub humidity: f32, // TODO: Make floating point independent
    pub temperature: f32, // TODO: Make floating point independent
}

impl<I2C, E> hih6030<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    /// Creates a new driver associated with an I2C peripheral
    ///
    /// You'll likely want to setup the device after this
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let hih6030 = hih6030 {
            i2c: i2c,
            address: 0x27,
            //state: [0; 8]
        };

        Ok(hih6030)
    }
    pub fn measure(&mut self) -> Result<Measurement, Error> {
        // Send a measurement request to the sensor
        self.meas_req().unwrap(); // TODO: Handel error
        // Read 4 bytes of data from the sensor
        let mut buf = [0;3];
        self.data_fetch(&mut buf).unwrap(); // TODO: Handel error
        // Decode the data to sensor status
        let status: u8 = (buf[0] & 0xC0) >> 6;
        // Calculate humidity
        let mut rh_count: u16 = (buf[0] & 0x3F) << 8 | buf[1];
        let mut rh: f32 = rh_count as f32 * 100.0 / 16382.0;
        // Calculate temperature
        let mut temp_count: u16 = buf[2] << 6 | buf[3] >> 2;
        let mut temp: f32 = temp_count as f32 * 165.0 / 16382.0 - 40.0;

        // Return with the measurement struct
        Ok(Measurement {
            match status {
            0 => Status::Ok,
            1 => Status::StaleData,
            2 => Status::CommandMode,
            3 => Status::Unknown,
            _ => Status::Unknown,
            },
            rh,
            temp,
        })

    }

    // TODO: Should be private
    fn meas_req(&mut self) -> Result<(), E> {
        let mut payload = [0; 1]; // TODO: SHOULD NOT SEND A BYTE
        payload[0] = 255;
        self.i2c.write(self.address, &payload)
        Ok()
    }
    // TODO: Should be private
    fn data_fetch(&mut self, mut buf: &mut [u8]) -> Result<(), E> {
        self.i2c.read(self.address, &mut buf)?;
        Ok()
    }

}