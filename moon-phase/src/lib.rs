/* A library that returns back moon phase given day and location */

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MoonPhasesData {
    apiversion: String,
    day: i32,
    month: i32,
    numphases: i32,
    phasedata: Vec<MoonPhase>,
    year: i32,
}

#[derive(Deserialize, Debug)]
struct MoonPhase {
    day: i32,
    month: i32,
    phase: String,
    time: String,
    year: i32,
}

pub async fn get_moon_phase(date: NaiveDate) -> Result<String, reqwest::Error> {
    let moon_phases_data: MoonPhasesData = reqwest::get(&format!(
        "https://aa.usno.navy.mil/api/moon/phases/date?date={}&nump=48",
        date.format("%Y-%m-%d")
    ))
    .await?
    .json::<MoonPhasesData>()
    .await?;

    // print version
    println!("API Version: {}", moon_phases_data.apiversion);
    // print number of phases
    println!("Number of phases: {}", moon_phases_data.numphases);
    // print day and month
    println!("Day: {}", moon_phases_data.day);
    println!("Month: {}", moon_phases_data.month);
    // print year
    println!("Year: {}", moon_phases_data.year);

    let phase = &moon_phases_data.phasedata[0].phase;
    // print day, month , year and time together
    println!(
        "Day: {}, Month: {}, Year: {}, Time: {}",
        moon_phases_data.phasedata[0].day,
        moon_phases_data.phasedata[0].month,
        moon_phases_data.phasedata[0].year,
        moon_phases_data.phasedata[0].time
    );
    Ok(phase.to_string())
}
