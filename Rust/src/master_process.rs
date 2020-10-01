#[allow(dead_code)]
mod channel;

use std::collections::HashMap;
//use zmq::Socket;
use std::clone::Clone;
use port_scanner::request_open_port;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value as JsonValue;

/* tuple to hold the address as a string, port as a number */
type AddressPort = (String, u16);

/* struct to hold channel information */
#[derive(Clone)]
pub struct ChannelInfo
{
   name: String,
   info: AddressPort,
   publishers: Vec<AddressPort>,
   subscribers: Vec<AddressPort>,
}

/* universal message format */
#[derive(Serialize, Deserialize)] 
pub struct Message 
{     
   pub messageType: char,            
   pub ip: String,  
   pub port: u16, 
   pub message: String, 
} 

pub struct MasterProcess
{
   //hash by channel name, store channel objects
   pub channels: HashMap<String, ChannelInfo>,
   pub ipAddress: String,
   pub port: u16,
}

impl MasterProcess
{
   /* Start the master process. This will be the main loop */
   pub fn start(mut self)
   {
      //set up the socket so we can connect to publishers and subscribers
      let context = zmq::Context::new();
      let repSocket = context.socket(zmq::REP).unwrap();
      let port = request_open_port().unwrap_or(0);
      repSocket
         .connect( &("tcp://0.0.0.0:".to_owned() + &port.to_string()) )
         //.connect("tcp://0.0.0.0:7000")
         .expect("failed binding socket");
      println!("repSocket bound");
      thread::sleep(Duration::from_millis(1));

      //get the port that we are bound to
      let lastEndpoint = match repSocket.get_last_endpoint()
      {
         Ok(lastEndpoint) => {
            match lastEndpoint {
               Ok(lastEndpoint) => lastEndpoint,
               Err(_e) => String::new(),
            }
         },
         Err(_e) => "failed".to_string(),
      };

      println!("{}", lastEndpoint);

      //start main loop
      loop 
      {
         //wait for a message to come in from a subscriber or publisher
         let mut msg = zmq::Message::new();
         repSocket.recv(&mut msg, 0).unwrap();      
      
         //get package as a string
         let msg_data = msg.as_str().unwrap();
         let msg_string = serde_json::from_str(msg_data);
   
         //deserialize into message struct
         let msg: Message = msg_string.unwrap();

         println!("Mode: {}", msg.messageType);
         println!("IP: {}", msg.ip);
         let channelName = msg.message;
         let nodeIP = msg.ip;
         let nodePort = msg.port;
         //let reqType = msg.messageType;

         //if the channel doesn't already exist, create it
         if self.channels.contains_key(&channelName) == false
         {
            self.channels.insert(channelName.clone(), MasterProcess::newChannel( self.ipAddress.clone(), channelName.clone() )) ;
         }

         //get the channel information
         let channelInfo = &self.channels[&channelName].clone();

         //convert the ip address of the channel to a byte array
         //let channelIP = &channelInfo.info.0.as_bytes();
         let channelPort = &channelInfo.info.1.to_be_bytes();
         
      
         //send data back to node
         let msg = Message { 
            messageType: 'M',
            ip: nodeIP,  
            port: nodePort, 
            message: channelName, 

          };
         let msg_str = serde_json::to_string(&msg);
         let serial_message: String = msg_str.unwrap();
         repSocket.send(&serial_message, 0).unwrap();

         /* if we want to exit, call break; */
      }; 
         

      
   }

   /********************** PRIVATE ******************/

   /* Creates a new channel process */
   fn newChannel(ipAddress: String, channelName: String) -> ChannelInfo
   {

      //pass assigned port into new channel
      let port = request_open_port().unwrap_or(0);
      thread::spawn(move || {
         let mut c = channel::Channel::new(port);
         //c.setMode(channel::ChannelMode::BLACKLIST);
         let mut terminate = false;
         while(!terminate)
         {
            c.main();
            //catch channel terminate
            //add to BLACKLIST or WHITELIST mid execution
            terminate = true;

         }
         
      });

      let contactInfo: AddressPort = ( ipAddress, port );
      let newChann = ChannelInfo { name: channelName, info: contactInfo, publishers: Vec::new(), subscribers: Vec::new(), };
      return newChann;
   }

   /* takes in a message from a pub/sub and decodes it */
   fn parseMessage(byte_msg: &Vec<u8>, ipString: &mut String, port: &mut u16, mode: &mut u8, channelName: &mut String)
   {
      //first 4 bytes are the sender ip address, so lets extract that
      *ipString = [byte_msg[0].to_string(),
                   byte_msg[1].to_string(),
                   byte_msg[2].to_string(), 
                   byte_msg[3].to_string()]
                   .join(".");

      //next two bytes are the port
      *port = ((byte_msg[4] as u16) << 8) | byte_msg[5] as u16;

      //next byte is the mode
      //0 for publisher, 1 for subscriber, 0 for node ping
      *mode = byte_msg[6];

      //next byte is reserved, do nothing

      //the rest of the bytes are the channel name, so just go until the byte vector has ended
      *channelName = String::from_utf8( byte_msg[7..].to_vec() ).unwrap();

   }


}

/* example usage for parseMessage 

let v: Vec<u8> = vec![1, 2, 3, 4, 0, 4, 10, 0, 240, 159, 146, 150];
let mut mode = 0;
let mut channelName: String = "".to_string();
let mut ipString: String = "".to_string();
let mut port: u16 = 0;
MasterProcess::parseMessage(&v, &mut ipString, &mut port, &mut mode, &mut channelName);
println!("Mode: {}", mode);
println!("IP: {}", ipString);
println!("Port: {}", port);
println!("Channel Name: {}", channelName);

*/