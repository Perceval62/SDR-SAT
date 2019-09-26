
use rtlsdr;
use std::vec::Vec;

fn main() {
    /* Get the total number of rtlsdr devices on the local machine */
    let number_of_devices = rtlsdr::get_device_count();

    /* this section could be replaced by a match*/
    if number_of_devices < 1 {
        panic!("There is no rtlsdr device on this machine.");
    }

    if number_of_devices == 1 {
        println!(
            "There is {} rtlsdr device(s) on this machine",
            number_of_devices
        );
    }

    /* open all devices and order them in a vector */
    let mut device_list: Vec<rtlsdr::RTLSDRDevice> = Vec::new();

    for i in 0..number_of_devices {
        device_list.push(rtlsdr::open(i).unwrap());
    }


}