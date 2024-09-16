use monome::{Monome, MonomeDeviceType};

pub fn initialize_monome(prefix: &str) -> Option<Monome> {
    match Monome::enumerate_devices() {
        Ok(devices) => {
            for d in devices.iter() {
                if d.device_type() == MonomeDeviceType::Grid {
                    return Monome::from_device(&d, prefix).ok();
                }
            }
        }
        Err(_) => {
            println!("No monome devices found");
        }
    }
    None
}