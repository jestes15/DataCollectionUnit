use data_models::{PressureUnits, TempUnits};

pub mod data_models;

fn main() {
    let firmware_data = data_models::FirmwareData {
        version: "1.32.5".to_string(),
        build_date: "27-DEC-2024".to_string(),
    };

    let mut temperature_data = data_models::TemperatureData {
        valid: true,
        temperature: 25.0,
        unit: TempUnits::Celsius,
    };

    let mut gryo_data = data_models::GryoData {
        valid: true,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut magnetometer_data = data_models::MagnetometerData {
        valid: true,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut accelerometer_data = data_models::AccelerometerData {
        valid: true,
        x: 0.0,
        y: -9.81,
        z: 0.0,
    };

    let mut pressure_data = data_models::PressureData {
        valid: true,
        pressure: 1.6,
        unit: PressureUnits::hPa,
    };

    let mut gps_data = data_models::GPSData {
        valid: true,
        latitude: 0.0,
        longitude: 0.0,
        speed: 30.0,
        time: "00:00:00".to_string(),
    };

    let mut data = data_models::Data {
        timestamp: 0,
        temperature: temperature_data,
        gryo: gryo_data,
        magnetometer: magnetometer_data,
        accelerometer: accelerometer_data,
        pressure: pressure_data,
        gps: gps_data,
    };

    println!("{}", serde_json::to_string(&data).unwrap());
}
