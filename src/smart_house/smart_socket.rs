pub struct SmartSocket {
    description: String,
    is_on: bool,
    power_consumption: f32,
}

impl SmartSocket {
    pub fn new(description: String) -> SmartSocket {
        SmartSocket {
            description,
            is_on: false,
            power_consumption: 0.0,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn is_turn_on(&self) -> bool {
        self.is_on
    }

    pub fn is_turn_off(&self) -> bool {
        !self.is_on
    }

    pub fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }

    pub fn get_description(&self) -> String {
        self.description.to_string()
    }
}

#[test]
fn test_create() {
    let mut sut = SmartSocket::new("test-description".to_string());

    assert_eq!("test-description".to_string(), sut.get_description());
    assert!(sut.is_turn_off());
    assert_eq!(0.0, sut.get_power_consumption());

    sut.turn_on();
    assert!(sut.is_turn_on());

    sut.turn_off();
    assert!(sut.is_turn_off());
}
