

extern crate serde_json;
extern crate serde;
//#[macro_use]
extern crate serde_derive;

use splay::SplayMap;
use splay::SplaySet;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;

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

#[derive(Serialize, Deserialize)]
pub struct ImboundMessage
{
     pub messageType: char,           
     pub ip: u32,
     pub port: u16,
     pub message: String,

}
#[derive(Serialize, Deserialize)]
pub struct OutboundMessage
{
     pub message: String,
}
impl Default for OutboundMessage
{
     fn default() -> OutboundMessage 
     {
          OutboundMessage
          {
               message: String::from(""),
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
        return format!("Name:{}\nMode:{:?}\n",self.name, self.mode);
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
     pub fn recieveDataOnly(&mut self, message: String)
     {
          self.info.add(message);
          return;
     }

     
     pub fn main(&mut self)
     {

          let context = zmq::Context::new();
          let responder = context.socket(zmq::REP).unwrap();

          let mut str1 = String::from("tcp://*:");
          let str_with_port = self.port.to_string();
          let address = [str1, str_with_port].concat();


          println!("value of s is {:?}", address);

          
          assert!(responder.bind(&address).is_ok());
      
          let mut msg = zmq::Message::new();

          
          loop 
          {
               responder.recv(&mut msg, 0).unwrap();
               //println!("Received {}", msg.as_str().unwrap());
               //inound is the deserialized struct 

               //data as string
               let data = msg.as_str().unwrap();
               let res = serde_json::from_str(data);

               //json deserialized stored inside p value
               let p: ImboundMessage = res.unwrap();
               if  p.messageType == 'd'
               {
                   // self.info.add((msg.as_str().unwrap());
               }
               if  p.messageType == 'r'
               {
                    let temp = self.info.get();
                    let m = OutboundMessage { message: temp };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               if  p.messageType == 's'
               {
                    let temp = String::from("STATUS REQUEST: Not Avalilible");
                    let m = OutboundMessage { message: temp };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               thread::sleep(Duration::from_millis(1000));
               



          }
     }
     fn crop_letters(s: &str, pos: usize) -> &str {
          match s.char_indices().skip(pos).next() {
              Some((pos, _)) => &s[pos..],
              None => "",
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
          pub fn get(&mut self) -> String
          {
               let mut retval = String::from("");
               for i in &self.info
               {
                    retval.push_str(i);
                    //retval = [retval, i].concat();
               }
               self.info.clear();
               return retval;
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