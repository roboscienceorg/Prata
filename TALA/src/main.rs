
#![allow(non_snake_case)]

//mod TEST_Channel;
mod channel;
mod master;
use std::{thread, time};
//use std::thread;

//use port_scanner::request_open_port;

const IPADDRESS: &str = "127.0.0.1";

fn main() {

    run_tests();
    //let mut line = String::new();
    //let _b1 = std::io::stdin().read_line(&mut line).unwrap();

}
fn run_tests()
{
    test_fifo();
    thread::sleep(time::Duration::from_millis(200));
    test_custom_fifo();
    thread::sleep(time::Duration::from_millis(200));
    test_broadcast();
    thread::sleep(time::Duration::from_millis(200));
    test_FIFO_and_Broadcast();
    thread::sleep(time::Duration::from_millis(200));
    test_invalid_host_ip()
    //thread::sleep(time::Duration::from_millis(200));
    //test_set_port_range()
}
#[allow(dead_code)]
fn test_set_port_range()
{
    //WIP
    println!("TEST: set_port_range");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();
    thread::sleep(time::Duration::from_millis(200));


    println!("TEST: set_port_range END");
}
#[allow(dead_code)]
fn test_invalid_host_ip()
{
    println!("TEST: invalid_host_ip");
    println!("---TEST: Expecting to see host fail error, If it prints, this tests passes. Consider boolean instead of print?");
    let m = master::Master {ipAddress: "207.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    thread::sleep(time::Duration::from_millis(200));


    println!("TEST: invalid_host_ip END");
}
#[allow(dead_code)]
fn test_fifo()
{
    println!("TEST: test_fifo");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelA".to_string(),"testingMessage".to_string());

    assert!(sub_.listen("ChannelA".to_string()) == "testingMessage", "TEST: test_fifo failed case 1");
    assert!(sub_.listen("ChannelA".to_string()) == "", "TEST: test_fifo failed case 2");
    m.terminate();
    println!("TEST: test_fifo END");
}
#[allow(dead_code)]
fn test_custom_fifo()
{

    println!("TEST: test_custom_fifo");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();

    m.createChannel(25566, "ChannelB".to_string(), "FIFO".to_string(), 500);

    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelB".to_string(),"testingMessage".to_string());

    assert_eq!(sub_.listen("ChannelB".to_string()),"testingMessage", "TEST: test_custom_fifo failed case 1");
    assert_eq!(sub_.listen("ChannelB".to_string()),"".to_string(), "TEST: test_custom_fifo failed case 2");
    m.terminate();
    println!("TEST: test_custom_fifo END");
}
#[allow(dead_code)]
fn test_broadcast()
{

    println!("TEST: test_broadcast");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();

    m.createChannel(25566, "ChannelC".to_string(), "BROADCAST".to_string(), 500);

    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelC".to_string(),"testingMessage".to_string());

    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 1");
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 2");

    m.terminate();
    println!("TEST: test_broadcast END");
}
#[allow(dead_code)]
fn test_FIFO_and_Broadcast()
{

    println!("TEST: FIFO_and_Broadcast");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();

    m.createChannel(25566, "ChannelC".to_string(), "BROADCAST".to_string(), 500);

    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelC".to_string(),"testingMessage".to_string());
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 1");
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 2");
    pub_.publish("Channel".to_string(),"m1".to_string());
    pub_.publish("Channel".to_string(),"m2".to_string());
    pub_.publish("Channel".to_string(),"m3".to_string());
    assert!(sub_.listen("Channel".to_string()) == "m1", "TEST: test_custom_fifo failed case 3");
    assert!(sub_.listen("Channel".to_string()) == "m2", "TEST: test_custom_fifo failed case 4");
    assert!(sub_.listen("Channel".to_string()) == "m3", "TEST: test_custom_fifo failed case 5");
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 6");
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 7");
    pub_.publish("ChannelC".to_string(),"new message".to_string());
    assert!(sub_.listen("ChannelC".to_string()) == "new message", "TEST: test_custom_fifo failed case 8");
    assert!(sub_.listen("ChannelC".to_string()) == "new message", "TEST: test_custom_fifo failed case 9");

    m.terminate();
    println!("TEST: FIFO_and_Broadcast END");
}