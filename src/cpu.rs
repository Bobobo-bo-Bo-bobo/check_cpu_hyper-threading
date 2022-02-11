use crate::constants;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct HyperThreadingResult {
    pub enabled: bool,
    pub cores: i64,
    pub siblings: i64,
}

pub fn is_hyper_threading_enabled() -> Result<HyperThreadingResult, Box<dyn Error>> {
    let enabled: bool;
    let fd = File::open(constants::CPUINFO)?;
    let mut buffer_reader = BufReader::new(fd);
    let mut content = String::new();

    buffer_reader.read_to_string(&mut content)?;

    let cores = get_cpu_cores(&content)?;
    let siblings = get_siblings(&content)?;

    if cores == siblings {
        enabled = false;
    } else {
        enabled = true;
    }

    Ok(HyperThreadingResult {
        enabled,
        cores,
        siblings,
    })
}

fn get_cpu_cores(raw: &str) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = -1;

    for line in raw.split('\n') {
        if let Some(v) = line.find(constants::CPU_CORES_STRING) {
            if v == 0 {
                let kv: Vec<&str> = line.split(':').collect();
                if kv.len() != 2 {
                    bail!("Invalid number of fields, got {} instead of 2", kv.len());
                }
                let value = kv[1].trim();
                result = value.parse::<i64>()?;
                break;
            }
        }
    }

    if result == -1 {
        bail!(
            "Can't find line starting with \"{}\" in content of {}",
            constants::CPU_CORES_STRING,
            constants::CPUINFO
        );
    }

    Ok(result)
}

fn get_siblings(raw: &str) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = -1;

    for line in raw.split('\n') {
        if let Some(v) = line.find(constants::SIBLINGS_STRING) {
            if v == 0 {
                let kv: Vec<&str> = line.split(':').collect();
                if kv.len() != 2 {
                    bail!("Invalid number of fields, got {} instead of 2", kv.len());
                }
                let value = kv[1].trim();
                result = value.parse::<i64>()?;
                break;
            }
        }
    }

    if result == -1 {
        bail!(
            "Can't find line starting with \"{}\" in content of {}",
            constants::SIBLINGS_STRING,
            constants::CPUINFO
        );
    }

    Ok(result)
}
