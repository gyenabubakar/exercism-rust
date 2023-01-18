// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PRODUCED_IN_HOUR_AT_SPEED_1: f64 = 221.0;

enum SuccessRatesPerSpeedRange {
    Range1To4,
    Range5To8,
    Range9To10,
}

fn get_success_rate(speed: u8) -> f64 {
    if speed >= 1 && speed <= 4 {
        1.0
    } else if speed >= 5 && speed <= 8 {
        0.9
    } else {
        0.77
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_produced = (speed as f64) * CARS_PRODUCED_IN_HOUR_AT_SPEED_1;
    get_success_rate(speed) * cars_produced
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production = production_rate_per_hour(speed) as u32;
    production / 60 as u32
}
