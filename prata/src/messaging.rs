use serde_json;
use serde;
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

/*
Unused code
Future development potentially to allow communiacation
to Channel for fifoALL mode. Where each pub/sub could
have its own fifo queue. The position would hold the 
publisher/subscriber position in the fifoALL queue and
the text would be the last value recieved.

This would work by a pusblisher requesting data, and
instread of just recieving a message string, it recieves
a message and position of where the next message will be
*/
#[derive(Serialize, Deserialize)]
pub struct PositionText
{
   pub position: u32,
   pub text: String,
}


/**
 * Unused Code
 * Deserialises JSON into struct
 */
#[allow(dead_code)]
pub fn JsonToTextPosition(json: String) -> PositionText
{
   let msg_string = serde_json::from_str(&json);

   //deserialize into message struct
   let msg: PositionText = msg_string.unwrap();
   return msg;
}

/**
 * Sends a TCP message to an IP and port
 * toAddress (String) - IP to send data to
 * toPort (u16) - port to send data to
 * message (Message) - Message to send
 */
#[allow(dead_code)]
pub fn send(toAddress: String, toPort: u16, message: Message) -> Message
{
   //Builds socket
   let context = zmq::Context::new();
   let responder = context.socket(zmq::REQ).unwrap();

   //Creates the url connection string
   let protocol = "tcp://".to_string();
   let str1 = String::from(toAddress);
   let str2 = String::from(":");
   let str_with_port = toPort.to_string();
   let address = [protocol, str1, str2, str_with_port].concat();

   //Connects the socket, consider removing assert!!! will crash
   assert!(responder.connect(&address).is_ok());

   //Creates message to send and serializes JSON
   let m = Message { messageType: message.messageType, ip: message.ip.to_string(), port: message.port,  message: message.message.to_string() };
   let res = serde_json::to_string(&m);
   let serial_message: String = res.unwrap();
   let mut msg = zmq::Message::new();

   //Sends message and listens for response
   responder.send(&serial_message, 0).expect("fail");
   responder.recv(&mut msg, 0).unwrap();

   //Unwraps and deserializes data into struct
   let data = msg.as_str().unwrap();
   let res = serde_json::from_str(data);
   let m = res.unwrap();
   
   return m;
}