
#[allow(dead_code)]
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

//use splay::SplayMap;
use splay::SplaySet;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value as JsonValue;
use std::collections::HashMap;

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
#[derive(Clone)]
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
impl Ports
{
     pub fn remove(&mut self, port: u16)
     {
          self.fullRange = false;
          self.portRange.remove(&port);
     }
     pub fn insert(&mut self, port: u16)
     {
          self.portRange.insert(port);
     }
     pub fn count(&mut self) -> usize
     {
          return self.portRange.len();
     }
}


/**
 * Represents an imbound message recieved from Publisher and Subscriber objs
 * messageType (char) - what request is made to the channel
 *                       D for data push
 *                       S for status pull
 *                       R for data pull
 *                       T for terminate loop
 * ip (String) - what ip is sending the message
 * port (u16) - what port is sending the message
 * message (String) - the message recieved
 * 
 * Defaults there is no defaults for this object
 * 
 * 
 */
#[derive(Serialize, Deserialize)] 
pub struct Message 
{     
     pub messageType: char,            
     pub ip: String,  
     pub port: u16, 
     pub message: String, 
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
 * 
 */

pub struct Channel
{

     pub mode: ChannelMode,
     pub name: String,
     pub ip: String,   
     pub port: u16,                     
     pub info: self::data::Information, 
     pub protocol: String,
     //maps an ip to its port range
     pub addressBook : HashMap<String,Ports>,  //STRING
     
}
impl Default for Channel
{
    fn default() -> Channel 
    {
        Channel
        {

               mode: ChannelMode::STANDARD,
               name: String::from("NoName"),
               ip: String::from("ImplementChannelIP"),
               port: 55555,
               info: self::data::Information::new(),
               protocol: String::from("tcp"),
               addressBook: HashMap::new()
        }
    }
}



/**
 * String override to:
 * "
 * Mode: STANDARD
 * Port: 55555
 * "
 */
impl ToString for Channel
{
     fn to_string(&self) -> String
     {
        return format!("\nName: {}\nMode: {:?}\nPort: {}\nProtocol: {}\n",self.name, self.mode, self.port, self.protocol);
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
    pub fn add(&mut self, ip: String )
    {
        self.addressBook.insert(ip, Default::default() );
    }

    pub fn remove(&mut self, ip: String )
    {
        self.addressBook.remove(&ip);
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
     
     pub fn addWithPorts(&mut self, ip: String, min: u16, max: u16 )
     {
          let mut ss = SplaySet::<u16>::new();
 

          for x in min..max
          {
              ss.insert(x);
          }
          let mut ports = Ports { fullRange: false, portRange: ss };
          self.addressBook.insert(ip, ports );
     
     }
     pub fn getPorts(&mut self, ip: String) -> &Ports
     {

          return (self.addressBook.get(&ip)).unwrap();
     }
     pub fn addPort(&mut self, ip: String, port: u16)
     {
          let  ports = (self.addressBook.get_mut(&ip)).unwrap();
          ports.insert(port);
          let y = ports.clone();
          self.addressBook.insert(ip, y);
     }

     pub fn removePort(&mut self, ip: String, port: u16)
     {
          let ports = (self.addressBook.get_mut(&ip)).unwrap();
          ports.remove(port);
          let y = ports.clone();
          self.addressBook.insert(ip, y);
     }
     /**
      * 
          Adds data to internal info

          param message (String) - data to add to info

          return void

      */
     pub fn addData(&mut self, message: String)
     {
          self.info.add(message);
          return;
     }

     pub fn getData(&mut self) -> String
     {
          return self.info.get();
     }

     pub fn getListed(&mut self) -> Vec<String>
     {
          let mut vec = Vec::new();
          for key in self.addressBook.keys() {
               vec.push(key.to_string());
           }
           return vec;
     }

     pub fn count(&mut self) -> usize
     {
          return self.getListed().len();
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
     fn validAddress(&mut self, ip: String, port: u16) -> bool
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

          //return true;s
     }
     
     pub fn setMode(&mut self, m: ChannelMode)
     {
          self.mode = m;
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
          //tcp://*:25565

          println!("value of s is {:?}", address);

          
          assert!(responder.bind(&address).is_ok());
          //ERROR if assert fails break
          let mut msg = zmq::Message::new();

          
          loop 
          {
               //read inbound messages
               responder.recv(&mut msg, 0).unwrap();
               //Can never return none cause it waits

               //data as string
               let data = msg.as_str().unwrap();
               let res = serde_json::from_str(data);

               //json deserialized stored inside p value
               let inbound: Message = res.unwrap();

               //white/black list check for valid credentials
               if self.validAddress(inbound.ip, inbound.port) == false
               {
                    //do nothing if invalid
               }
               else if  inbound.messageType == 'D'
               {
                    //add data
                    //use CLASS ADD FUNCTION
                    self.info.add(inbound.message);
               }
               else if  inbound.messageType == 'R'
               {
                    //send data
                    let temp = self.info.get();
                    let m = Message { messageType: 'D', ip: self.ip.to_string(), port: self.port,  message: temp };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               else if  inbound.messageType == 'S'
               {
                    //send status
                    let temp = String::from("STATUS REQUEST: Not Avalilible");
                    let m = Message { messageType: 'S', ip: self.ip.to_string(), port: self.port,  message: temp };

                    let res = serde_json::to_string(&m);
                    //let res = serde_json::to_string(&self.status);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               else if inbound.messageType == 'T'
               {
                    //terminate channel listening and return to caller
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

