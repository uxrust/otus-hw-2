pub struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    pub fn new() -> Thermometer {
        Thermometer { temperature: 0.0 }
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[test]
fn test_create() {
    let sut = Thermometer::new();
    assert_eq!(0.0, sut.get_temperature());
}
