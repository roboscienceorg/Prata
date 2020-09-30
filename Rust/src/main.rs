

mod channel;

fn main() {
    //let mut c = channel::Channel { mode: channel::ChannelMode::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
    //let mut c = channel::Channel { ..Default::default()};
    let mut c = channel::Channel::new(55555);
    c.getListed();
    c.add(String::from("192.168.0.1"));
    c.add(String::from("192.168.0.1"));
    c.add(String::from("192.168.0.1"));
    let listed = c.getListed();
    println!("KEYS:\n{:?}", listed);
    c.add(String::from("192.168.0.1"));
    c.add(String::from("192.122.0.1"));
    c.add(String::from("192.334.0.1"));
    c.add(String::from("192.0.0.1"));
    c.add(String::from("192.66.0.1"));
    let listed = c.getListed();
    println!("KEYS:\n{:?}", listed);
    //println!("Heello, world!");
    //println!("{}", c.to_string());

    //c.main();
    
}
