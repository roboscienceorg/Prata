

mod channel;

fn main() {
    let mut c = channel::Channel { mode: channel::ChannelMode::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
    c.add(44);
    println!("Hello, world!");
    println!("{}", c.to_string());
}
