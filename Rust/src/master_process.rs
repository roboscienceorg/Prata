#[allow(dead_code)]
mod channel;

use std::collections::HashMap;
//use zmq::Socket;
use std::clone::Clone;
use port_scanner::request_open_port;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::panic;
#[path = "messaging.rs"] mod messaging;
//use serde_json::Result;
//use serde_json::Value as JsonValue;

/* tuple to hold the address as a string, port as a number */
type AddressPort = (String, u16);

/* struct to hold channel information */
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
      let mut full_address = "tcp://".to_string();
      full_address.push_str(&self.ipAddress);
      full_address.push_str(&":");
      full_address.push_str(&self.port.to_string());
      let context = zmq::Context::new();
      let repSocket = context.socket(zmq::REP).unwrap();
      //let port = request_open_port().unwrap_or(0);
          panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
      let fail_status = panic::catch_unwind(|| {repSocket.bind( &(full_address) ).expect("fail");});
      match fail_status
      {
         Ok(_fail_status) => println!("Construct host: Master({}, {})", self.ipAddress.to_string(), self.port),
         Err(_) => {println!("Invalid IP and Port combination, cannot host"); return;},
      }
      //println!("{:?}", repSocket.expect());
         //.connect("tcp://0.0.0.0:7000")
         //.expect("failed binding socket XXX");
      thread::sleep(Duration::from_millis(1));

      //get the port that we are bound to
      let _lastEndpoint = match repSocket.get_last_endpoint()
      {
         Ok(lastEndpoint) => {
            match lastEndpoint {
               Ok(lastEndpoint) => lastEndpoint,
               Err(_e) => String::new(),
            }
         },
         Err(_e) => "failed".to_string(),
      };


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

      //   println!("mp: {}", msg.messageType);
         //let reqType = msg.messageType;
         if  msg.messageType == 'T'
         {
            //terminate host
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            repSocket.send(&serial_message, 0).unwrap();

            //terminate by returning this thread
            return;
         }
         else if  msg.messageType == 'D' || msg.messageType == 'd'
         {
            //publisher disconnect
            //d is for publisher D is for subscriber
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            repSocket.send(&serial_message, 0).unwrap();

            //take off internal list
            //message is string of channel name

            //get channelInfo obj
            //let temp_s = msg.message.to_string();
            let chan_info = (self.channels.get_mut(&msg.message)).unwrap();
            let addr_prt: AddressPort = (msg.ip.to_string(), msg.port);
            if msg.messageType == 'D'
            {
               let index = chan_info.subscribers.iter().position(|x| *x == addr_prt).unwrap();
               chan_info.subscribers.remove(index);
            }
            else if msg.messageType == 'd'
            {
               let index = chan_info.publishers.iter().position(|x| *x == addr_prt).unwrap();
               chan_info.publishers.remove(index);
            }



         }
         else if msg.messageType == 'C' || msg.messageType == 'c'
         {
            //publisher requesting connection to channel
            //check if channel exists
            let channel_port: u16;
            let channel_exists = self.channels.contains_key(&msg.message);
            if channel_exists == true
            {
               let chan_info = (self.channels.get_mut(&msg.message)).unwrap();
               if msg.messageType == 'c'
               {
                  chan_info.publishers.push( (msg.ip.to_string(), msg.port) );
               }
               else if msg.messageType == 'C'
               {
                  chan_info.subscribers.push( (msg.ip.to_string(), msg.port) );
               }
               channel_port = chan_info.info.1;
            }
            else
            {
               //get port .
               channel_port = request_open_port().unwrap_or(0);
               //make channel and insert it into hash map
               let mut chan_info = MasterProcess::newChannel(self.ipAddress.to_string(), channel_port, msg.message.to_string());
               if msg.messageType == 'c'
               {
                  chan_info.publishers.push(  (msg.ip.to_string(), msg.port) );
               }
               else if msg.messageType == 'C'
               {
                  chan_info.subscribers.push(  (msg.ip.to_string(), msg.port) );
               }
               self.channels.insert(msg.message.to_string(), chan_info);

            }


            //set correct addresses
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: channel_port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            repSocket.send(&serial_message, 0).unwrap();
         }
         else if msg.messageType == 'J'
         {
            //handle returning a json in message
            let me = serde_json::to_string(&self);
            let meserl: String = me.unwrap();
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: meserl.to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            repSocket.send(&serial_message, 0).unwrap();
         }
         else if msg.messageType == 'R'
         {
            //Rmove a channel listed
            //println!("mp: in R");
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            self.terminateChannel(msg.message);
            repSocket.send(&serial_message, 0).unwrap();
            // println!("mp: sending ACK");
            //
            // println!("mp: sending ACK complete");
         }

         /* if we want to exit, call break; */
      };



   }

   /********************** PRIVATE ******************/
   fn terminateChannel( &mut self, name: String)
   {
      if  self.channels.contains_key(&name)
      {
         let chanInfo = self.channels.get(&name).unwrap();
         let p = self.port;
         let m = messaging::Message { messageType: 'T', ip: self.ipAddress.to_string(), port: p,  message: "".to_string() };
         //println!("terminating channel {}: {}. {}", name, chanInfo.info.0.to_string(), chanInfo.info.1);
         messaging::send(chanInfo.info.0.to_string(), chanInfo.info.1, m);
         self.channels.remove(&name);
      }
   }
   /* Creates a new channel process */
   fn newChannel(ipAddress: String, port: u16, channelName: String) -> ChannelInfo
   {

      //pass assigned port into new channel

      let s = ipAddress.to_string();
      let p = port;

      thread::spawn(move || {
         let mut c = channel::Channel::new(s.to_string(), p);
         //c.setMode(channel::ChannelMode::BLACKLIST);
         let mut terminate = false;
         while terminate == false
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
    #[allow(dead_code)]
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
