

mod channel;

fn main() {
    //let mut c = channel::Channel { mode: channel::ChannelMode::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
    //let mut c = channel::Channel { ..Default::default()};
    let mut c = channel::Channel::new(55555);
    //println!("Heello, world!");
    //println!("{}", c.to_string());

    //c.main();
    
}
