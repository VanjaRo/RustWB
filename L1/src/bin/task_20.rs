struct CelsiusThermometer;

impl CelsiusThermometer {
    fn get_temperature(&self) -> f64 {
        25.0 // Температура в Цельсиях (пример)
    }
}

// New trait: measuring temperature in Fahrenheit
trait FahrenheitThermometer {
    fn get_temperature_f(&self) -> f64;
}

// Adapter: convert temperature from celsius to fahrenheit
struct ThermometerAdapter {
    celsius_thermometer: CelsiusThermometer,
}

impl ThermometerAdapter {
    fn new(celsius_thermometer: CelsiusThermometer) -> Self {
        ThermometerAdapter {
            celsius_thermometer,
        }
    }
}

// Implementing a new interface using an adapter
impl FahrenheitThermometer for ThermometerAdapter {
    fn get_temperature_f(&self) -> f64 {
        let celsius = self.celsius_thermometer.get_temperature();
        celsius * 9.0 / 5.0 + 32.0 // Convert to Fahrenheit
    }
}

fn main() {
    let old_thermometer = CelsiusThermometer;

    let adapter = ThermometerAdapter::new(old_thermometer);

    println!(
        "Temperature in Fahrenheit: {:.2}",
        adapter.get_temperature_f()
    );
}
