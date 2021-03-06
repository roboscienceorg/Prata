
#![allow(non_snake_case)]

mod channel;
mod master;
use std::{thread, time};

//IP address to use for testing
const IPADDRESS: &str = "127.0.0.1";

//Main initiates rust test cases
fn main() {

    run_tests();

}

/**
 * Runs Rust test cases
 */
fn run_tests()
{
    //Unit tests for Channel
    basic_test();
    thread::sleep(time::Duration::from_millis(200));
    remove_ip_test();
    thread::sleep(time::Duration::from_millis(200));
    add_data_test();
    thread::sleep(time::Duration::from_millis(200));
    remove_data_test();
    thread::sleep(time::Duration::from_millis(200));
    add_ip_ports_test();

    //Integration tests
    test_fifo();
    thread::sleep(time::Duration::from_millis(200));
    test_custom_fifo();
    thread::sleep(time::Duration::from_millis(200));
    test_broadcast();
    thread::sleep(time::Duration::from_millis(200));
    test_FIFO_and_Broadcast();
    thread::sleep(time::Duration::from_millis(200));
    test_invalid_host_ip();
    thread::sleep(time::Duration::from_millis(200));


    
    //Error when publishing to channel, deleting channel then publishing
    //to the same channel. Error occurs from hanging TCP socket as it never
    //gets a reply from the deleted channel's original ip and port.
    //test_delete_channel_after_pub();
    
}

/**
 * Tests setting port ranges for Channel
 */
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

/**
 * tests using an invalid IP address for Master
 */
#[allow(dead_code)]
fn test_invalid_host_ip()
{
    println!("TEST: invalid_host_ip");
    println!("---TEST: Expecting to see host fail error. TEST PASSES if Invalid IP error occurs!");
    let m = master::Master {ipAddress: "207.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    thread::sleep(time::Duration::from_millis(200));


    println!("TEST: invalid_host_ip END");
}

/**
 * Tests FIFO functionality of Channel
 */
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

/**
 * Tests publishing to a deleted channel after a Publisher
 * has already send data to it once
 */
#[allow(dead_code)]
fn test_delete_channel_after_pub()
{
    println!("TEST: test_delete_channel_after_pub");
    let m = master::Master {ipAddress: IPADDRESS.to_string(), port: 25565, threading: true};
    m.host();
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelA".to_string(),"testingMessage".to_string());

    assert!(sub_.listen("ChannelA".to_string()) == "testingMessage", "TEST: test_fifo failed case 1");
    assert!(sub_.listen("ChannelA".to_string()) == "", "TEST: test_fifo failed case 2");

    m.removeChannel("ChannelA".to_string());
    thread::sleep(time::Duration::from_millis(200));
    pub_.publish("ChannelA".to_string(),"testingMessage".to_string());
    assert!(sub_.listen("ChannelA".to_string()) == "testingMessage", "TEST: test_fifo failed case 1");
    assert!(sub_.listen("ChannelA".to_string()) == "", "TEST: test_fifo failed case 2");

    m.terminate();
    println!("TEST: test_delete_channel_after_pub END");
}

/**
 * Tests creating a custom FIFO channel
 */
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

/**
 * Tests broadcast functionality
 */
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

/**
 * Tests FIFO and Broadcast functionality
 */
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
fn add_ip_ports_test()
{
    println!("TEST: Channel - Add IP and Ports");
    let config = channel::ChannelConfiguration::new(IPADDRESS.to_string(), 25565, "CUSTOM".to_string(), "fifo".to_string(), 20);
    let mut c = channel::Channel::new(config);
    c.add("192.168.0.0".to_string());
    let ports = c.getPorts("192.168.0.0".to_string());

    assert!(ports.fullRange == true, "TEST: Added single port to list, full range was not set")
    
}
fn remove_data_test()
{
    println!("TEST: Channel - Remove Data");
    let config = channel::ChannelConfiguration::new(IPADDRESS.to_string(), 25565, "CUSTOM".to_string(), "fifo".to_string(), 20);
    let mut c = channel::Channel::new(config);

    let data = c.getData();
    assert!(data == "", "TEST: Constructs with data already inside FAIL");

}
fn add_data_test()
{
    println!("TEST: Channel - Add Data");
    let config = channel::ChannelConfiguration::new(IPADDRESS.to_string(), 25565, "CUSTOM".to_string(), "fifo".to_string(), 20);
    let mut c = channel::Channel::new(config);
    c.addData("data element 1".to_string());
    c.addData("data element 2".to_string());
    c.addData("data element 3".to_string());
    c.addData("data element 4".to_string());

    let j = c.getData();
    assert!(j == "data element 1".to_string(), "TEST: get data fail");

    c.addData("the big\nhouse".to_string());
    c.addData("\"what\" are you doing".to_string());
    c.addData("\t\t\r\n".to_string());
    c.addData("2 9 {{}}".to_string());

    assert!(c.getData() == "data element 2", "TEST: get data 2 fail")
}
fn basic_test()
{
    println!("TEST: Channel - Basic");
    let config = channel::ChannelConfiguration::new(IPADDRESS.to_string(), 25565, "CUSTOM".to_string(), "fifo".to_string(), 20);
    let mut c = channel::Channel::new(config);
    let listed = c.getListed();
    assert!(listed.len() == 0, "TEST: Default constructor starts with ip listed");

    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 1, "TEST: Adding does not increase size by 1");

    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 1, "TEST: Adding duplicate increases size by 1");

    c.add("192.168.0.0".to_string());
    c.add("192.168.0.1".to_string());
    c.add("192.168.0.2".to_string());
    c.add("192.168.0.3".to_string());
    c.add("192.168.0.0".to_string());
    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 4, "TEST: Adding additional adresses then duplicates fails");

}
fn remove_ip_test()
{
    println!("TEST: Channel - IP removal");
    let config = channel::ChannelConfiguration::new(IPADDRESS.to_string(), 25565, "CUSTOM".to_string(), "fifo".to_string(), 20);
    let mut c = channel::Channel::new(config);

    c.add("192.168.0.1".to_string());
    c.add("192.168.0.2".to_string());
    c.add("192.168.0.3".to_string());
    c.add("192.168.0.0".to_string());
    c.add("192.168.0.0".to_string());
    c.remove("192.168.0.0".to_string());
    c.remove("192.168.0.2".to_string());

    assert!(c.count() == 2, "TEST: Removing fail");

    c.remove("192.168.0.1".to_string());
    c.remove("192.168.0.3".to_string());

    assert!(c.count() == 0, "TEST: Removing fail");

    c.remove("192.168.0.1".to_string());
    c.remove("192.168.0.3".to_string());

    assert!(c.count() == 0, "TEST: Removing from empty fail");

}
