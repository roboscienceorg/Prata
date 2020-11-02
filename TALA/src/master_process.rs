#[allow(dead_code)]
mod channel;

use std::collections::HashMap;
use std::clone::Clone;
use port_scanner::request_open_port;
use std::thread;
use serde::{Deserialize, Serialize};
use std::panic;
#[path = "messaging.rs"] mod messaging;

/* tuple to hold the address as a string, port as a number */
type AddressPort = (String, u16);

/**
 * Just a struct that holds the statistics about a particular channel.
 * numReceived (u32) - number of messages the channel has received from a publisher
 * numSent (u32) - number of messages the channel has sent to subscribers
 * numStored (u32) - number of messages the channel has stored currently
 * pubTimestamps (HashMap<String, u128>) - saves the timestamp of the last message received from a publisher.
 *                                         The key is a string concatenated from the ip address and port.
 * subTimestamps (HashMap<String, u128>) - saves the timestamp of the last request received from a subscriber.
 *                                         The key is a string concatenated from the ip address and port.
 */
 #[derive(Clone)]
 #[derive(Serialize, Deserialize)]
 pub struct ChannelStatistics
 {
     pub numReceived: u32,
     pub numSent: u32,
     pub numStored: u32,
     pub pubTimestamps: HashMap<String, u128>,
     pub subTimestamps: HashMap<String, u128>,
 }

/**
 * Struct to hold information about a specific channel.
 * name (String) - name of the channel
 * info (AddressPort) - a tuple that holds the address and port the channel is available at
 * publishers (Vec<AddressPort>) - List of address port combinations for every publisher for this channel
 * subscribers (Vec<AddressPort>) - List of address port combinations for every subscriber for this channel
 * channelStatistics (ChannelStatistics) - Struct that holds statistics about the channel
 */
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct ChannelInfo
{
   name: String,
   info: AddressPort,
   publishers: Vec<AddressPort>,
   subscribers: Vec<AddressPort>,
   channelStatistics: ChannelStatistics,
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


/**
 * Struct defining the master process object.
 * channels (HashMap<String, ChannelInfo>) - collection of information on every channel
 * ipAddress (String) - IP address this process is available at
 * port (u16) - port this process is available at
 * portRange ( (u16, u16) ) - reserved range of ports for channels? not sure, please correct
 * nextPort (u16) - idk
 * isCustomRange (bool) - idk
 * 
 */
#[derive(Serialize, Deserialize)]
pub struct MasterProcess
{
   //hash by channel name, store channel objects
   pub channels: HashMap<String, ChannelInfo>,
   pub ipAddress: String,
   pub port: u16,
   pub portRange: (u16, u16),
   pub nextPort: u16,
   pub isCustomRange: bool,
}

/**
 * Defines the functions for the master process object.
 */
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
          panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
      let fail_status = panic::catch_unwind(|| {repSocket.bind( &(full_address) ).expect("fail");});
      match fail_status
      {
         Ok(_fail_status) => {},
         Err(_) => {println!("---Invalid IP and Port combination, cannot host"); return;},
      }

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

         if  msg.messageType == 'T'
         {
            //terminate host
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            self.reply(m,&repSocket);

            let origination_port = request_open_port().unwrap_or(0);
            for (_, value) in self.channels.iter()
            {
               let m = messaging::Message { messageType: 'T', ip: self.ipAddress.to_string(), port: origination_port,  message: "".to_string() };
               messaging::send(value.info.0.to_string(), value.info.1, m);
            }

            //terminate by returning this thread
            return;
         }
         else if  msg.messageType == 'D' || msg.messageType == 'd'
         {
            //publisher disconnect
            //d is for publisher D is for subscriber
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            self.reply(m,&repSocket);

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
               //get port
               channel_port = self.getPort();
               //make channel and insert it into hash map
               let config = channel::ChannelConfiguration::new(self.ipAddress.to_string(), channel_port, msg.message.to_string(), channel::Channel::getDefaultType() , 500);
               let mut chan_info = MasterProcess::newChannel(config);
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
            self.reply(m,&repSocket);
         }
         else if msg.messageType == 'J'
         {
            //returns the current state of the master process in a serialized json
            //loop through all channels, and ping for status request
            let mut keys = Vec::new();
            let mut vecStats: Vec<ChannelStatistics> = Vec::new();
            for (key, val) in self.channels.iter_mut() {
               let address = &val.info.0;
               let port = &val.info.1;
               
               //bind socket to new address
               let context = zmq::Context::new();
               let responder = context.socket(zmq::REQ).unwrap();
               let protocol = "tcp://".to_string();
               let str1 = String::from(address);
               let str2 = String::from(":");
               let str_with_port = port.to_string();
               let address = [protocol, str1, str2, str_with_port].concat();
               assert!(responder.connect(&address).is_ok());

               //build message
               let m = Message { messageType: 'S', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
               let res = serde_json::to_string(&m);
               let serial_message: String = res.unwrap();
               let mut msg = zmq::Message::new();

               //send request and receive status report
               responder.send(&serial_message, 0).expect("fail");
               responder.recv(&mut msg, 0).unwrap();

               //deserialize struct and get channel statistics struct
               let data = msg.as_str().unwrap();
               let res = serde_json::from_str(data);
               let chanStats = res.unwrap();

               keys.push(key);
               vecStats.push(chanStats);  
            }
            
            //update all of the channels statistics structs for each channel
            let mut i = 0;
            for channel in self.channels.values_mut()
            {
               channel.channelStatistics = vecStats[i].clone();
               i+=1;
            }
            

            //serialize master process and send back json
            let me = serde_json::to_string(&self);
            let meserl: String = me.unwrap();
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: meserl.to_string() };
            self.reply(m,&repSocket);
         }
         else if msg.messageType == 'R'
         {
            //Remove a channel listed
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            self.terminateChannel(msg.message);
            repSocket.send(&serial_message, 0).unwrap();
         }
         else if msg.messageType == 'N'
         {
            //creates new channel of specified type
            //if it already exists, do nothing
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            let res = serde_json::to_string(&m);
            let serial_message: String = res.unwrap();
            
            let chan_data = msg.message.as_str();
            let chan_info = serde_json::from_str(chan_data);
            let mut config: channel::ChannelConfiguration = chan_info.unwrap();


            let channel_exists = self.channels.contains_key(&config.name);
            if channel_exists == false
            {

               let name_of_channel = config.name.to_string();

               if channel::Channel::getSupportedTypes().contains(&config.stylet.to_string()) == false
               {
                  println!("\nERROR: Channel type \"{}\" not supported; using channel default \"{}\"\n", config.stylet.to_string(), channel::Channel::getDefaultType());
                  config.stylet = channel::Channel::getDefaultType();
               }
               
               let chan_info = MasterProcess::newChannel(config);
               self.channels.insert(name_of_channel, chan_info);
            }

            repSocket.send(&serial_message, 0).unwrap();

         }
         else if msg.messageType == 'P'
         {
            //message is port range to set
            let mut lines = msg.message.split(':');
            let first = (lines.next().unwrap()).parse::<u16>().unwrap();
            let second = (lines.next().unwrap()).parse::<u16>().unwrap();
            self.nextPort = first;
            self.portRange = ( first, second );
            self.isCustomRange = true;

            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            self.reply(m,&repSocket);
         }
         else if msg.messageType == 'O'
         {
            //print supported channel types
            println!("{:?}", channel::Channel::getSupportedTypes());
            let m = Message { messageType: 'A', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
            self.reply(m,&repSocket);
         }

         /* if we want to exit, call break; */
      };
   }

   /********************** PRIVATE ******************/

   /*
   *  Terminates a channel
   */
   fn terminateChannel( &mut self, name: String)
   {
      if  self.channels.contains_key(&name)
      {
         let chanInfo = self.channels.get(&name).unwrap();
         let p = self.port;
         let m = messaging::Message { messageType: 'T', ip: self.ipAddress.to_string(), port: p,  message: "".to_string() };
         messaging::send(chanInfo.info.0.to_string(), chanInfo.info.1, m);
         self.channels.remove(&name);
      }
   }

   /* 
   * serializes and sends a reply to the requesting socket 
   */
   fn reply (&mut self, m: Message, repSocket: &zmq::Socket)
   {
      let res = serde_json::to_string(&m);
      let serial_message: String = res.unwrap();
      repSocket.send(&serial_message, 0).unwrap();
   }

   #[allow(dead_code)]
   fn getPort(&mut self) -> u16
   {
      let port;
      if self.isCustomRange == false
      {
         return request_open_port().unwrap_or(0);
      }

      if self.nextPort <= self.portRange.1
      {
         port = self.nextPort;
         self.nextPort = self.nextPort + 1;
      }
      else
      {

         port = self.portRange.0;
         self.nextPort = self.portRange.0 + 1;
      }

      return port;
   }
   /* 
   * Creates a new channel process. Takes in a channelconfiguration object to determine channel behavior.
   */
   fn newChannel(config: channel::ChannelConfiguration) -> ChannelInfo
   {
      //pass assigned port into new channel
      let n = (config.name).to_string();
      let p = config.port;
      let ip = (config.ip).to_string();
      let config_ = config;

      thread::spawn(move || {
         let mut c = channel::Channel::new(config_);
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

      let contactInfo: AddressPort = ( ip.to_string(), p );
      let newChann = ChannelInfo { 
         name: n.to_string(), 
         info: contactInfo, 
         publishers: Vec::new(), 
         subscribers: Vec::new(), 
         channelStatistics: ChannelStatistics {numReceived: 0, numSent: 0, numStored: 0, pubTimestamps: HashMap::new(), subTimestamps: HashMap::new()},
      };
      return newChann;
   }
}