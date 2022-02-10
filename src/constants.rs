pub const NAME: &str = "check_cpu_hyper-threading";
pub const VERSION: &str = "1.0.0";

pub const NAGIOS_OK: i32 = 0;
pub const NAGIOS_WARNING: i32 = 1;
pub const NAGIOS_CRITICAL: i32 = 2;
pub const NAGIOS_UNKNOWN: i32 = 3;

pub const CPUINFO: &str = "/proc/cpuinfo";
pub const CPU_CORES_STRING: &str = "cpu cores";
pub const SIBLINGS_STRING: &str = "siblings";
