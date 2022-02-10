use crate::constants;

pub struct NagiosResult {
    pub status: i32,
    pub message: String,
}

pub fn process_result(ht_mode: &str, ht_enabled: bool, warn_mode: bool) -> NagiosResult {
    let status: i32;
    let message: String;

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

    NagiosResult { status, message }
}
