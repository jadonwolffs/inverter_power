use std::io;
fn main() {
    println!("Please input the power of the device:");
    let mut power_string = String::new();
    io::stdin()
        .read_line(&mut power_string)
        .expect("Failed to read line");
    let power = power_string.trim().parse::<f32>().unwrap();

    println!("Please input the amount of time you intend to run the device by battery:");
    let mut time_string = String::new();
    io::stdin()
        .read_line(&mut time_string)
        .expect("Failed to read line");
    let time: f32 = time_string.trim().parse::<f32>().unwrap();

    println!(
        "Please input the efficiency of your inverter (as a double e.g. 0.85 for 85% efficiency):"
    );
    let mut efficiency_string = String::new();
    io::stdin()
        .read_line(&mut efficiency_string)
        .expect("Failed to read line");
    let efficiency: f32 = efficiency_string.trim().parse::<f32>().unwrap();

    println!("Please input the total voltage of the battery/batteries powering your inverter:");
    let mut voltage_string = String::new();
    io::stdin()
        .read_line(&mut voltage_string)
        .expect("Failed to read line");
    let voltage: f32 = voltage_string.trim().parse::<f32>().unwrap();

    let wh_device: f32 = power * time;
    let wh_battery: f32 = wh_device / efficiency;
    let ah_battery: f32 = wh_battery / voltage;

    println!("W: {}", power);
    println!("Wh of device: {}", wh_device);
    println!("Wh of battery: {}", wh_battery);
    println!("Ah of battery used: {}", ah_battery);
    println!("Press [enter] to close ");
    let mut end_string = String::new();
    io::stdin()
        .read_line(&mut end_string)
        .expect("Failed to read line");
    if end_string.eq_ignore_ascii_case("r") {
        main();
    }
}
