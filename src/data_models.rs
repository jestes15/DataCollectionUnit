use serde::{Deserialize, Serialize};

/* Firmware Struct */

#[derive(Serialize, Deserialize, Debug)]
pub struct FirmwareData {
    pub version: String,
    pub build_date: String,
}

/* Temperature Struct and Enumeration for Units */

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum TempUnits {
    Celsius = 1,
    Fahrenheit = 2,
    Kelvin = 3,
    Rankine = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemperatureData {
    pub valid: bool,
    pub temperature: f32,
    pub unit: TempUnits,
}

/* Gyroscope Sturct and Enumeration for Units */

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GyroUnits {
    dps = 1, /* Degrees per second */
    rps = 2, /* Rotations per second */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GryoData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub unit: GyroUnits,
}

/* Magnetometer Sturct and Enumeration for Units */

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MegnetometerUnits {
    uT = 1, /* Microtesla */
    mG = 2, /* Milligauss */
    G = 3,  /* Gauss */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MagnetometerData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub unit: MegnetometerUnits,
}

/* Acceleromter Sturct and Enumeration for Units */

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum AccelerometerUnits {
    mps2 = 1,  /* Meters per second squared */
    ftps2 = 2, /* Feet per second squared */
    g = 3,     /* Standard gravity */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccelerometerData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub unit: AccelerometerUnits,
}

/* Pressure Sturct and Enumeration for Units */

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PressureUnits {
    Pa = 1,
    hPa = 2,
    psi = 3,
    atm = 4,
    torr = 5,
    bar = 6,
    mmHg = 7,
    Ba = 8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PressureData {
    pub valid: bool,
    pub pressure: f32,
    pub unit: PressureUnits,
}

/* GPS Sturct */

#[derive(Serialize, Deserialize, Debug)]
pub struct GPSData {
    pub valid: bool,
    pub latitude: f32,
    pub longitude: f32,
    pub speed: f32,
    pub time: String,
}

/* Aggregate Data Struct */

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub timestamp: u64,
    pub firmware: FirmwareData,
    pub temperature: TemperatureData,
    pub gryo: GryoData,
    pub magnetometer: MagnetometerData,
    pub accelerometer: AccelerometerData,
    pub pressure: PressureData,
    pub gps: GPSData,
}
