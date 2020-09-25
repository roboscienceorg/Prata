

mod channel;

fn main() {
    let mut c = channel::Channel { mode: channel::ChannelMode::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
    c.add(44);
    println!("Heello, world!");
    println!("{}", c.to_string());
    println!("Hello, world!");
    c.main();
    
}
