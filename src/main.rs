use smart_house::smart_socket::SmartSocket;
use smart_house::thermometer::Thermometer;

mod smart_house;

fn main() {
    let mut smart_socket = SmartSocket::new("test-smart-socket".to_string());

    smart_socket.turn_on();
    smart_socket.turn_off();

    let _description = smart_socket.get_description();
    let _power_consumption = smart_socket.get_power_consumption();
    let _is_turn_on = smart_socket.is_turn_on();
    let _is_turn_off = smart_socket.is_turn_off();

    let thermometer = Thermometer::new();
    let _temperature = thermometer.get_temperature();
}
