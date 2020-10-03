
#![allow(non_snake_case)]
mod TEST_Channel;
mod channel;
mod master;



fn main() {
    //println!("ChannelTests");

    TEST_Channel::test();
    let m = master::Master::new();
    m.host();

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
