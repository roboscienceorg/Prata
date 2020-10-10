

use serde_json;
use serde;
//use serde_derive;
use serde::{Deserialize, Serialize};
/* universal message format */
#[derive(Serialize, Deserialize)]
pub struct Message
{
   pub messageType: char,
   pub ip: String,
   pub port: u16,
   pub message: String,
}



pub fn send(toAddress: String, toPort: u16, message: Message) -> Message
{
     let context = zmq::Context::new();
     let responder = context.socket(zmq::REQ).unwrap();

     let protocol = "tcp://".to_string();
     let str1 = String::from(toAddress);
     let str2 = String::from(":");
     let str_with_port = toPort.to_string();
     let address = [protocol, str1, str2, str_with_port].concat();

     assert!(responder.connect(&address).is_ok());
     let m = Message { messageType: message.messageType, ip: message.ip.to_string(), port: message.port,  message: message.message.to_string() };

     let res = serde_json::to_string(&m);
     let serial_message: String = res.unwrap();
     let mut msg = zmq::Message::new();
     responder.send(&serial_message, 0).unwrap();
     responder.recv(&mut msg, 0).unwrap();
     //data as string
     let data = msg.as_str().unwrap();
     let res = serde_json::from_str(data);
     //json deserialized stored inside p value
     let m: Message = res.unwrap();
     return m;
}
