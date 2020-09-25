


//use std::collections::Ve
use splay::SplayMap;
use splay::SplaySet;
use std::thread;
use std::time::Duration;

#[derive(Debug)]


pub enum ChannelMode
{
    STANDARD,
    WHITELIST,
    BLACKLIST
}
impl Default for ChannelMode
{
     fn default() -> Self {ChannelMode::STANDARD}
}

/*
pub struct MinMax
{
    pub min: u16,
    pub max: u16,
}
impl Default for MinMax
{
    fn default() -> MinMax 
    {
        //sets the min to min and max to max
        MinMax { min: u16::MIN, max: u16::MAX}
    }

}
*/
pub struct Ports
{
     pub fullRange: bool,
     pub portRange: SplaySet<u16>,
}
impl Default for Ports
{
     fn default() -> Ports 
     {
          Ports
          {
               fullRange: true,
               portRange: SplaySet::new()
          }
     }
}


pub struct Channel
{
     pub mode: ChannelMode,             //mode of channel
     pub name: String,                  //name of channel
     pub port: u16,                     //port for listening
     pub info: self::data::Information, //information storage
     //maps an ip to its port range
     pub addressBook : SplayMap<i32,Ports>,  //blacklist stuff
     


}
impl Default for Channel
{
    fn default() -> Channel 
    {
        Channel
        {
            mode: ChannelMode::STANDARD,
            name: String::from("1"),
            port: 55555,
            info: self::data::Information::new(),
            addressBook: SplayMap::new()
        }
    }
}
impl ToString for Channel
{
     fn to_string(&self) -> String
     {
        return format!("Name:{} Mode:{:?}",self.name, self.mode);
     }
}
impl Channel
{
    //adds ip address to addressbook with default port range 0-max
    pub fn add(&mut self, ip: i32 )
    {
        self.addressBook.insert(ip, Default::default() );
    }

     //adds ip address to addressbook with inclusvie port range min-max
     pub fn addWithPorts(&mut self, ip: i32, min: u16, max: u16 )
     {
          let mut ss: SplaySet<u16>;
 

          for x in min..max
          {
               //&ss.clear();
          }
          let mut ports = Ports { fullRange: false, ..Default::default() };
          self.addressBook.insert(ip, ports );
     
     }

     pub fn decode(&mut self, message: String)
     {
          /* stack overflow code convert string to binary

          let name = "Jake".to_string();
          let mut name_in_binary = "".to_string();
      
          // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
          // I only called clone() because this for loop takes ownership
          for character in name.clone().into_bytes() {
              name_in_binary += &format!("0{:b} ", character);
          }
          println!("\"{}\" in binary is {}", name, name_in_binary);
          */
          let messageType = message.chars().next().unwrap();

          if (messageType == 'b')
          {
               println!("\"{}\" at index {} in binary is {:?}", message, 0, messageType);
          }
               /*
          let mut index = 0;
          let mut name_in_binary = "".to_string();
          for character in message.clone().into_bytes() {
               name_in_binary = format!("0{:b} ", character);
               println!("\"{}\" at index {} in binary is {}", message, index, name_in_binary);
               index = index + 1;
          }
          */
     }
     pub fn main(&mut self)
     {
          println!("sup");
          let context = zmq::Context::new();
          let responder = context.socket(zmq::REP).unwrap();

          let mut str1 = String::from("tcp://*:");
          let str_with_port = self.port.to_string();
          let address = [str1, str_with_port].concat();
          println!("value of s is {:?}", address);
          let temp = String::from("abcdef");
          self.decode(temp);
          return;
          
          assert!(responder.bind(&address).is_ok());
      
          let mut msg = zmq::Message::new();
          loop {
              responder.recv(&mut msg, 0).unwrap();
              println!("Received {}", msg.as_str().unwrap());
              thread::sleep(Duration::from_millis(1000));
              responder.send("World", 0).unwrap();



          }
     }


}




mod data
{
     use std::collections::VecDeque;

     pub struct Information
     {
          pub info: self::VecDeque<String>,
     }


     impl Information
     {
          pub fn add(&mut self, bytes: String)
          {
               self.info.push_back(bytes);
          }
     }
     impl Information
     {
          pub fn new() -> Information
          {
               return Information { info: VecDeque::new()};
          }
     }
}