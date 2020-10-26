
#![allow(non_snake_case)]

//mod TEST_Channel;
mod channel;
mod master;

//use std::thread;

//use port_scanner::request_open_port;

fn main() {

    // Print the ip addresses and dns servers of all adapters:
    //TEST_Channel::test();
    test_fifo();
    test_broadcast();
    test_custom_fifo();
    
    //test_fifo();
    //test_broadcast();
    //println!("stuff = {:?}", x);
    //let m = master::Master::new();
    /*
    let m = master::Master {ipAddress: "192.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    m.getChannelTypes();
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();
    println!("{}", sub_.to_string());
    println!("{}", pub_.to_string());
    m.createChannel(25566, "test".to_string(), "FIFO".to_string(), 20);
    println!("{}", m.serialize());
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    pub_.connect("num2".to_string());
    pub_.publish("num2".to_string(),"testing message 1=======".to_string());

    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    sub_.connect("num2".to_string());
    println!("listen 1 {}", sub_.listen("num2".to_string()));
    println!("{}", m.serialize());
    */
}

fn test_fifo()
{
    println!("TEST: test_fifo");
    let m = master::Master {ipAddress: "192.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();
    //println!("{:?}", m.serialize());
    pub_.publish("ChannelA".to_string(),"testingMessage".to_string());
    //println!("{:?}", m.serialize());
    assert!(sub_.listen("ChannelA".to_string()) == "testingMessage", "TEST: test_fifo failed case 1");
    assert!(sub_.listen("ChannelA".to_string()) == "", "TEST: test_fifo failed case 2");
    m.terminate();
}
fn test_custom_fifo()
{
    //let mut line = String::new();
    println!("TEST: test_custom_fifo");
    let m = master::Master {ipAddress: "192.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    //println!("{:?}", m.serialize());
    m.createChannel(25566, "ChannelB".to_string(), "FIFO_yo".to_string(), 500);
    //println!("{:?}", m.serialize());
    //let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();
    //let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    pub_.publish("ChannelB".to_string(),"testingMessage".to_string());
    //let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    assert_eq!(sub_.listen("ChannelB".to_string()),"testingMessage", "TEST: test_custom_fifo failed case 1");
    assert_eq!(sub_.listen("ChannelB".to_string()),"".to_string(), "TEST: test_custom_fifo failed case 2");
    m.terminate();
}
fn test_broadcast()
{

    println!("TEST: test_broadcast");
    let m = master::Master {ipAddress: "192.168.0.122".to_string(), port: 25565, threading: true};
    m.host();
    println!("{:?}", m.serialize());
    m.createChannel(25566, "ChannelC".to_string(), "BROADCAST".to_string(), 500);
    println!("{:?}", m.serialize());
    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.publish("ChannelC".to_string(),"testingMessage".to_string());
    //println!("{:?}", sub_.listen("ChannelC".to_string()));
    //println!("{:?}", sub_.listen("ChannelC".to_string()));
    //println!("{:?}", m.serialize());
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 1");
    assert!(sub_.listen("ChannelC".to_string()) == "testingMessage", "TEST: test_custom_fifo failed case 2");
    m.terminate();
}
