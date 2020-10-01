
#![allow(non_snake_case)]
mod TEST_Channel;
mod channel;
mod master;



fn main() {
    //println!("ChannelTests");
    //TEST_Channel::test();
    let m = master::Master::new();
    m.host();

    //c.main();
    
}
