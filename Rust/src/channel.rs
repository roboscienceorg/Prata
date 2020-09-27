

extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use splay::SplayMap;
use splay::SplaySet;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;

#[derive(Debug)]



/**
 * Represents Channel mode
 * 
 * STANDARD  - No white or blacklist
 * BLACKLIST - ip's and ports added to the channel are banned
 * WHITELIST - ip's and ports added to the channel are the only ones allowed to talk
 * 
 * Defaults to STANDARD
 */
pub enum ChannelMode
{
    STANDARD,
    WHITELIST,
    BLACKLIST,
}
impl Default for ChannelMode
{
     fn default() -> Self {ChannelMode::STANDARD}
}



/**
 * Represents ports for an ip address
 * fullRange (boolean) - true if all ports should be included
 * PortRange (SplaySet) - Contains ports included
 * 
 * Defaults to fullrange = true
 * Defaults to portRange = {empty set} 
 * 
 * Note - fullrange is meant to be used when all the ports are on the list
 *        if many are on the list, this data structure becomes large
 * Note - SplaySet could be removed for a hash set instead
 * 
 * 
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


/**
 * Represents an imbound message recieved from Publisher and Subscriber objs
 * messageType (char) - what request is made to the channel
 *                       D for data push
 *                       S for status pull
 *                       R for data pull
 * ip (u32) - what ip is sending the message
 * port (u16) - what port is sending the message
 * message (String) - the message recieved
 * 
 * Defaults there is no defaults for this object
 * 
 * 
 */
#[derive(Serialize, Deserialize)]
pub struct InboundMessage
{
     pub messageType: char,           
     pub ip: u32,
     pub port: u16,
     pub message: String,
}


/**
 * Represents an outbound message to publishers and subscribers
 * message (String) - the message to send back
 * 
 * Defaults to message = ""
 * 
 * 
 */
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

/**
 * Holds channel information
 * mode (ChannelMode) - Enum representing the mode of the channel
 * name (String) - Name given to the channel
 * port (u16) - Port the channel listens on for pub and sub objects
 * info (Information) - Holds data the channel has recieved
 * protocol (String) - Must be lowercase udp or tcp
 * addressBook (SplayMap<u32,Ports>) - used for ip and port lookup
 *                      for black/whitelist
 * 
 * Defaults to mode = STANDARD
 * Defaults to name = "NoName"
 * Defaults to port = 555555
 * Defaults to info = {empty}
 * Defaults to protocol = "tcp"
 * Defaults to addressBook = {empty}
 * 
 * Note the addressbook could be implemented with a traditinal map
 *        if the speed measured is slow on larger scales
 * 
 * 
 */
pub struct Channel
{
     pub mode: ChannelMode,             
     pub name: String,                  
     pub port: u16,                     
     pub info: self::data::Information, 
     pub protocol: String,
     //maps an ip to its port range
     pub addressBook : SplayMap<u32,Ports>,  
     
}
impl Default for Channel
{
    fn default() -> Channel 
    {
        Channel
        {
            mode: ChannelMode::STANDARD,
            name: String::from("NoName"),
            port: 55555,
            info: self::data::Information::new(),
            protocol: String::from("tcp"),
            addressBook: SplayMap::new()
        }
    }
}



/**
 * String override to:
 * "
 * Name: NoName
 * Mode: STANDARD
 * Port: 55555
 * "
 */
impl ToString for Channel
{
     fn to_string(&self) -> String
     {
        return format!("\nName: {}\nMode: {:?}\nPort: {}\n",self.name, self.mode, self.port);
     }
}
impl Channel
{
     /**
      * CONSTRUCTOR
      *
      */
     pub fn new(port_: u16) -> Channel
     {
          return Channel { port: port_, ..Default::default() };
     }
    /**
     * adds ip address to addressbook with default port range 0-max
     * 
     * param ip (u32) - ip to add to list
     * 
     * return void
     * 
     * */
    pub fn add(&mut self, ip: u32 )
    {
        self.addressBook.insert(ip, Default::default() );
    }

    /**
     * Adds ip to list with port range
     * 
     * param ip (u32) - ip address to add to list
     * param min (u16) - min port to add inclusive
     * param max (u16) - max port to add inclusive
     * 
     * return void
     */
     
     pub fn addWithPorts(&mut self, ip: u32, min: u16, max: u16 )
     {
          let mut ss = SplaySet::<u16>::new();
 

          for x in min..max
          {
              ss.insert(x);
          }
          let mut ports = Ports { fullRange: false, portRange: ss };
          self.addressBook.insert(ip, ports );
     
     }

     /**
      * 
          Adds data to internal info

          param message (String) - data to add to info

          return void

      */
     pub fn recieveDataOnly(&mut self, message: String)
     {
          self.info.add(message);
          return;
     }


     /**
      * Validates an ip address and port
      *
      * param ip (u32) - ip address to validate
      * param port (u16) - port to validate
      *
      * return true - valid credentials to recieve from
      * return false - invalid credentials to receive from
      *
      */
     pub fn validAddress(&mut self, ip: u32, port: u16) -> bool
     {
          
          //blacklist
          match self.mode
          {
               ChannelMode::BLACKLIST =>
               {
                    let ports = (self.addressBook.get(&ip)).unwrap();
                    //CRASH?? if none
                    /*
                    if ports == None
                    {

                         //ip was not in addressbook
                         return true;
                    }
                    */
                    if ports.fullRange == true
                    {
                         return false;
                    }
                    return !ports.portRange.contains(&port)
               }
               ChannelMode::WHITELIST =>
               {
                    let ports = (self.addressBook.get(&ip)).unwrap();
                    //CRASH?? if none
                    /*
                    if ports == None
                    {
                         //ip was not in addressbook
                         return false;
                    }
                    */
                    if ports.fullRange == true
                    {
                         return true;
                    }
                    return ports.portRange.contains(&port)
               }
               ChannelMode::STANDARD =>
               {
                    return true;
               }
          }

          return true;
     }
     
     /**
      * Main loop for channel. Runs nessesary processes
      *
      * param none
      *
      * return void - if function returns, channel listening has halted
      *
      */
     pub fn main(&mut self)
     {

          let context = zmq::Context::new();
          let responder = context.socket(zmq::REP).unwrap();

          let protocol = self.protocol.to_string();
          let str1 = String::from("://*:");
          let str_with_port = self.port.to_string();
          let address = [protocol, str1, str_with_port].concat();


          println!("value of s is {:?}", address);

          
          assert!(responder.bind(&address).is_ok());
      
          let mut msg = zmq::Message::new();

          
          loop 
          {
               //read inbound messages
               responder.recv(&mut msg, 0).unwrap();

               //data as string
               let data = msg.as_str().unwrap();
               let res = serde_json::from_str(data);

               //json deserialized stored inside p value
               let inbound: InboundMessage = res.unwrap();

               //white/black list check for valid credentials
               if self.validAddress(inbound.ip, inbound.port) == false
               {
                    //do nothing if invalid
               }
               else if  inbound.messageType == 'D'
               {
                    //add data
                    self.info.add(inbound.message);
               }
               else if  inbound.messageType == 'R'
               {
                    //send data
                    let temp = self.info.get();
                    let m = OutboundMessage { message: temp };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               else if  inbound.messageType == 'S'
               {
                    //send status
                    let temp = String::from("STATUS REQUEST: Not Avalilible");
                    let m = OutboundMessage { message: temp };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               else if inbound.messageType == 'T'
               {
                    //terminate channel and return to caller
                    return;
               }

               thread::sleep(Duration::from_millis(1000));
               
          }
     }

}



/**
 * Data class that holds information sent to channel object
 */
mod data
{
     use std::collections::VecDeque;


     /**
      * Holds data sent to class in FIFO structure
      *
      */
     pub struct Information
     {
          pub info: self::VecDeque<String>,
     }
     impl Information
     {
          /**
          * Adds a string to the fifo structure
          *
          * param bytes (String) - adds the string to the internal info object
          *
          * return none
          */
          pub fn add(&mut self, bytes: String)
          {
               self.info.push_back(bytes);
          }
          /**
          * gets a string to the fifo structure
          *
          * param none
          *
          * return (String) - All data from FIFO
          */
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
          /**
          * New call to return new object
          *
          * param none
          *
          * return (Information) - A blank information object
          */
          pub fn new() -> Information
          {
               return Information { info: VecDeque::new()};
          }
     }

}

