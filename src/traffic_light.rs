enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Time {
    fn time(&self) -> u8;
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match *self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 8,
        }
    }
}

#[test]
fn test_traffic_light() {
    println!("Red light lasts for {} seconds", TrafficLight::Red.time());
    println!(
        "Green light lasts for {} seconds",
        TrafficLight::Green.time()
    );
    println!(
        "Yellow light lasts for {} seconds",
        TrafficLight::Yellow.time()
    );
    assert_eq!(TrafficLight::Green.time(), 40)
}
