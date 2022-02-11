use crate::constants;
use crate::cpu;

pub struct NagiosResult {
    pub status: i32,
    pub message: String,
    pub perfdata: String,
}

pub fn process_result(
    ht_mode: &str,
    ht_status: cpu::HyperThreadingResult,
    warn_mode: bool,
) -> NagiosResult {
    let status: i32;
    let message: String;
    let perfdata: String;
    let ht_enabled = ht_status.enabled;

    if ht_mode == "on" {
        if ht_enabled {
            status = 0;
            message = "OK - Hyper-threading is on".to_string();
        } else if warn_mode {
            message = "WARNING - Hyper-threading is off".to_string();
            status = constants::NAGIOS_WARNING;
        } else {
            message = "CRITICAL - Hyper-threading is off".to_string();
            status = constants::NAGIOS_CRITICAL;
        }
    } else if ht_enabled {
        if warn_mode {
            message = "WARNING - Hyper-threading is on".to_string();
            status = constants::NAGIOS_WARNING;
        } else {
            message = "CRITICAL - Hyper-threading is on".to_string();
            status = constants::NAGIOS_CRITICAL;
        }
    } else {
        status = 0;
        message = "OK - Hyper-threading is off".to_string();
    }

    if ht_enabled {
        perfdata = format!(
            "cpus={} siblings={} hyper_threading={}",
            ht_status.cores, ht_status.siblings, 1
        );
    } else {
        perfdata = format!(
            "cpus={} siblings={} hyper_threading={}",
            ht_status.cores, ht_status.siblings, 0
        );
    }
    NagiosResult {
        status,
        message,
        perfdata,
    }
}
