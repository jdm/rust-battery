extern crate battery;

fn main() {
    for (idx, bat) in battery::get().enumerate() {
        println!("Device:\t\t\t{}", idx);
        println!("vendor:\t\t\t{}", bat.vendor().unwrap_or("N/A"));
        println!("model:\t\t\t{}", bat.model().unwrap_or("N/A"));
        println!("battery");
        println!("  state:\t\t{}", bat.state());
        println!("  energy:\t\t{:.2} Wh", bat.energy());
        println!("  energy-full:\t\t{:.2} Wh", bat.energy_full());
        println!("  energy-full-design:\t{:.2} Wh", bat.energy_full_design());
        println!("  energy-rate:\t\t{:.2} Wh", bat.energy_rate());
        println!("  voltage:\t\t{:.2} V", bat.voltage());
        match bat.state() {
            battery::State::Discharging => {
                println!("  time-to-empty\t\t{} seconds",
                         bat.time_to_empty()
                             .and_then(|d| Some(d.as_secs()))
                             .unwrap_or(0));
            },
            battery::State::Charging => {
                println!("  time-to-full\t\t{} seconds",
                         bat.time_to_full()
                             .and_then(|d| Some(d.as_secs()))
                             .unwrap_or(0));
            },
            _ => {},
        }
        println!("  percentage:\t\t{:.2}%", bat.percentage());
        println!("  temperature:\t\t{:.2} °C", bat.temperature());
        println!("  technology:\t\t{}", bat.technology());
    }
}