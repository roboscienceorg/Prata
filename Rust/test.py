import TALA as tl
import time

ip = "127.0.0.1".to_string();
let port = 25565;
let m = master::connect(ip.to_string(), port);

let mut line = String::new();
m.host(true);


let mut b1 = std::io::stdin().read_line(&mut line).unwrap();


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


m.terminate()

exit()
