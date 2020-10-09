
#![allow(non_snake_case)]

//mod TEST_Channel;
mod channel;
mod master;

//use std::thread;

//use port_scanner::request_open_port;

fn main() {

    // Print the ip addresses and dns servers of all adapters:



    //println!("stuff = {:?}", x);
    //let m = master::Master::new();
    let m = master::Master {ipAddress: "192.168.0.122".to_string(), port: 25565, threading: true};
    m.host();

    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();
    println!("{}", sub_.to_string());
    println!("{}", pub_.to_string());
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    pub_.connect("test".to_string());
    pub_.publish("test".to_string(),"testing message 1=======".to_string());

    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    sub_.connect("test".to_string());
    println!("listen 1 {}", sub_.listen("test".to_string()));
    //
    

        /*
    //println!("ChannelTests");

    TEST_Channel::test();


    //let m = master::Master::new();
    //let m = master::Master {ipAddress: "127.0.0.1".to_string(), port: 10819};

    let ip = "127.0.0.1".to_string();
    let port = 25565;
    let mut m = master::connect(ip.to_string(), port);

    let mut line = String::new();

    //m.setThreading(true);
    m.host();

    std::io::stdin().read_line(&mut line).unwrap();


    let mut sub_ = m.subscriber();
    let mut pub_ = m.publisher();

    pub_.connect("test".to_string());
    pub_.publish("test".to_string(),"testing message 1=======".to_string());


    sub_.connect("test".to_string());
    println!("listen 1 {}", sub_.listen("test".to_string()));
    pub_.publish("test".to_string(),"testing message2 ==========".to_string());
    println!("listen 2 {}", sub_.listen("test".to_string()));
    println!("listen 3 {}", sub_.listen("test".to_string()));
    //m.host(true);


    println!("Back to main from hosting");

    println!("{:?}", m.serialize());

    println!("Break1:");
    std::io::stdin().read_line(&mut line).unwrap();


    m.terminate();

    println!("Finished mains");

    let mut publisher = m.publisher();
    let channel = "X92.FM".to_string();
    let message = "FirstMessage".to_string();
    publisher.connect(channel);
    publisher.publish(channel, message);

    let mut subscriber = m.subscriber();
    let data = subscriber.listen(channel);

    println!("{} = FirstMessage",data.to_string());
    */

}
