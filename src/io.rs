#[path = "body.rs"]
mod body;

use crate::body::Body;
use std::error::Error;
use std::fs::File;
use byteorder::WriteBytesExt;
// This trait adds methods to writeable types
use byteorder::LittleEndian;

pub fn write_file(path: &str, bodies: &Vec<Body>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for i in 0..bodies.len() {
        file.write_f64::<LittleEndian>(bodies[i].x)?;
        file.write_f64::<LittleEndian>(bodies[i].y)?;
        file.write_f64::<LittleEndian>(bodies[i].z)?;
        file.write_f64::<LittleEndian>(bodies[i].vx)?;
        file.write_f64::<LittleEndian>(bodies[i].vy)?;
        file.write_f64::<LittleEndian>(bodies[i].vz)?;
        file.write_f64::<LittleEndian>(bodies[i].ax)?;
        file.write_f64::<LittleEndian>(bodies[i].ay)?;
        file.write_f64::<LittleEndian>(bodies[i].az)?;
    }
    Ok(())
}

pub fn read_csv(path: &str) -> Result<Vec<Body>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    let mut bodies: Vec<Body> = Vec::new();
    let au: f64 = 1.5e11;
    let m_sol: f64 = 2e30;
    let day: f64 = 24.0 * 60.0 * 60.0;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        bodies.push(Body {
            m: record[1].parse::<f64>().unwrap() * m_sol,
            x: record[2].parse::<f64>().unwrap() * au,
            y: record[3].parse::<f64>().unwrap() * au,
            z: record[4].parse::<f64>().unwrap() * au,
            vx: record[5].parse::<f64>().unwrap() * au / day,
            vy: record[6].parse::<f64>().unwrap() * au / day,
            vz: record[7].parse::<f64>().unwrap() * au / day,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            softening: 0.001,
        });
    }
    return Ok(bodies);
}