use std::net::UdpSocket;
use std::{thread, time};
mod clock_control;
mod heart_control;
mod matrix_control; 

fn main() {
    test_heart();
}

fn test_strip(){
    let mut strip_control = matrix_control::MatrixControl{
        socket: UdpSocket::bind("192.168.1.2:4220").expect("couldn't bind to address"),
        data_arr: Vec::with_capacity(1600), 
        num_pixels: 500
    };
}

fn test_clock(){

    // Construct our object. 
    let clk_control = clock_control::ClockControl{
                        off_cmd: [40, 40, 50, 65, 0], 
                        on_cmd: [40, 40, 50, 65, 1],
                        socket: UdpSocket::bind("192.168.1.2:4220").expect("couldn't bind to address")
                    };

    // Test functions. 
    clk_control.off();
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    clk_control.on();
    thread::sleep(one_seccond);
}

fn test_heart(){
    // Construct our heart. 
    let heart_control = heart_control::HeartControl{
        socket: UdpSocket::bind("192.168.1.2:4280").expect("couldn't bind to address")
    };

    // Test heart parameters. 
    //heart_control.lock();
    
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    
    heart_control.unlock();
    heart_control.lamp_off();
}