
#![allow(non_snake_case)]
mod TEST_Channel;
mod channel;
mod master;
use std::thread;


fn main() {
    //println!("ChannelTests");

    TEST_Channel::test();
    let m = master::Master::new();

    let sub_ = m.subscriber();
    let pub_ = m.publisher();

    m = m.host();

    //code

    m.terminate();
    println!("Finished mains");
    /*
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
