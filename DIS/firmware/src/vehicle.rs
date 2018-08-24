#![deny(unsafe_code)]
#![allow(non_snake_case)]

extern crate std;

use std::fs::File;
use std::io::{BufReader,BufRead};


pub struct VehicleConfiguration
{
    // gearing ration
    pub gearRatio : f32,
    // diameter of the drive wheel in m
    pub driveWheelDiameter : f32,
}

impl VehicleConfiguration {
    pub fn new() -> VehicleConfiguration {
        VehicleConfiguration {
            gearRatio: 0.0,
            driveWheelDiameter: 0.0,
        }
    }
}

pub fn LoadVehicleConfiguration(filePath : & str) -> VehicleConfiguration {
    
    let fileHndl = File::open(filePath).unwrap();
    let reader = BufReader::new(& fileHndl);

    let mut config = VehicleConfiguration::new();

    for line in reader.lines() {
        let mut l = line.unwrap();

        if l.starts_with("#") || l.trim().len() == 0 {
            continue;
        }

        let confPair : Vec<_> = l.split("=").collect();

        if confPair.len() != 2 {
            panic!("Error while reading vehicle configuration from '{}'. Line '{} is not a valid configuration pair. 'Key = Value' expected.", filePath, l);
        }

        match confPair[0].trim() {
            "GearRatio" => config.gearRatio = confPair[1].trim().parse::<f32>().unwrap(),
            "DriveWheelDiameter" => config.driveWheelDiameter = confPair[1].trim().parse::<f32>().unwrap(),
            _  => println!("Error while reading vehicle configuration from '{}'. Unknown key '{}' found.'", filePath, confPair[1]),
        }
    }

    config
}


pub struct VehicleData {
    // current RPM of the motor
    pub engineRPM : i32,
    // current charge of the battery in percent
    pub batteryCharge : i32,
}


pub fn CalculateDrivingSpeed(config : & VehicleConfiguration, data : & VehicleData) -> f32 {

    let wheelRPM = data.engineRPM as f32 * config.gearRatio;
    let meterPerMin = wheelRPM * config.driveWheelDiameter;
    ((meterPerMin * 60.0)/1000.0)
}