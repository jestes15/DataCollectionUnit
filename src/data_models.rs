use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum TempUnits {
    Celsius = 1,
    Fahrenheit = 2,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
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
pub struct FirmwareData {
    pub version: String,
    pub build_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemperatureData {
    pub valid: bool,
    pub temperature: f32,
    pub unit: TempUnits,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GryoData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MagnetometerData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccelerometerData {
    pub valid: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PressureData {
    pub valid: bool,
    pub pressure: f32,
    pub unit: PressureUnits,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPSData {
    pub valid: bool,
    pub latitude: f32,
    pub longitude: f32,
    pub speed: f32,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub timestamp: u64,
    pub temperature: TemperatureData,
    pub gryo: GryoData,
    pub magnetometer: MagnetometerData,
    pub accelerometer: AccelerometerData,
    pub pressure: PressureData,
    pub gps: GPSData,
}
