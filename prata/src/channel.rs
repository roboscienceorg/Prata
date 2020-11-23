
#[path = "messaging.rs"] mod messaging;
#[allow(dead_code)]
extern crate serde_json;
extern crate serde;
extern crate serde_derive;
use std::collections::VecDeque;
use splay::SplaySet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};


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
#[allow(dead_code)]
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
     #[allow(dead_code)]
     pub fn remove(&mut self, port: u16)
     {
          self.fullRange = false;
          self.portRange.remove(&port);
     }
      #[allow(dead_code)]
     pub fn insert(&mut self, port: u16)
     {
          self.portRange.insert(port);
     }
     #[allow(dead_code)]
     pub fn count(&mut self) -> usize
     {
          return self.portRange.len();
     }
}


/**
 * Represents an inbound message received from Publisher and Subscriber objs
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
 * Just a struct that holds the statistics about a particular channel.
 * numReceived (u32) - number of messages the channel has received from a publisher
 * numSent (u32) - number of messages the channel has sent to subscribers
 * numStored (u32) - number of messages the channel has stored currently
 * pubTimestamps (HashMap<String, u128>) - saves the timestamp of the last message received from a publisher.
 *                                         The key is a string concatenated from the ip address and port.
 * subTimestamps (HashMap<String, u128>) - saves the timestamp of the last request received from a subscriber.
 *                                         The key is a string concatenated from the ip address and port.
 */
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
 * Note the addressbook could be implemented with a traditional map
 *        if the speed measured is slow on larger scales
 *
 *
 *
 */
pub struct Channel
{
     pub mode: ChannelMode,
     pub styles: String,
     pub name: String,
     pub ip: String,
     pub port: u16,
     pub info: self::data::Information,
     pub protocol: String,
     //maps an ip to its port range
     pub addressBook : HashMap<String,Ports>,  //STRING
     pub limit: u32,
     pub channelStatistics: ChannelStatistics,
}
impl Default for Channel
{
    fn default() -> Channel
    {
        Channel
        {
               mode: ChannelMode::STANDARD,
               styles: String::from("fifo"),
               name: String::from("NoName"),
               ip: String::from("ImplementChannelIP"),
               port: 55555,
               limit: 500,
               info: self::data::Information::new(),
               protocol: String::from("tcp"),
               addressBook: HashMap::new(),
               channelStatistics: ChannelStatistics {numReceived: 0, numSent: 0, numStored: 0, pubTimestamps: HashMap::new(), subTimestamps: HashMap::new()},
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChannelConfiguration
{
   pub ip: String,
   pub port: u16,
   pub name: String,
   pub stylet: String,
   pub messageLimit: u32,
}
impl Default for ChannelConfiguration
{
    fn default() -> ChannelConfiguration
    {
         ChannelConfiguration
        {

               ip: String::from("0.0.0.0"),
               port: 0,
               name: String::from("DEFAULT"),
               stylet: String::from("fifo"),
               messageLimit: 500,
        }
    }
}
impl ChannelConfiguration
{
     #[allow(dead_code)]
     pub fn new(ip_: String, port_: u16, name_: String, style_: String, limit_: u32) -> ChannelConfiguration
     {
          return ChannelConfiguration{ip: ip_, port: port_, name: name_, stylet: style_, messageLimit: limit_};
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
      #[allow(dead_code)]
     pub fn new(config: ChannelConfiguration) -> Channel
     {

          let data_obj;
          if config.stylet == "BROADCAST"
          {
               data_obj = self::data::Information { info: VecDeque::new(), limit: 1, _deleteOnPull: false};
          }
          else
          {
               data_obj = self::data::Information { info: VecDeque::new(), limit: 500, _deleteOnPull: true};
          }

          return Channel { 
               port: config.port, 
               ip: config.ip, 
               styles: config.stylet.to_string(), 
               limit: config.messageLimit, 
               name: config.name, 
               info: data_obj, 
               mode: ChannelMode::STANDARD,
               protocol: String::from("tcp"), 
               addressBook: HashMap::new(),
               channelStatistics: ChannelStatistics {numReceived: 0, numSent: 0, numStored: 0, pubTimestamps: HashMap::new(), subTimestamps: HashMap::new()},
          };
   
     }
     #[allow(dead_code)]
     pub fn getSupportedTypes() -> Vec<String>
     {
          let mut vec = Vec::new();
          vec.push("FIFO".to_string());
          vec.push("BROADCAST".to_string());

          return vec;
     }
     #[allow(dead_code)]
     pub fn getDefaultType() -> String
     {
          return "FIFO".to_string();
     }
    /**
     * adds ip address to addressbook with default port range 0-max
     *
     * param ip (u32) - ip to add to list
     *
     * return void
     *
     * */
     #[allow(dead_code)]
    pub fn add(&mut self, ip: String )
    {
        self.addressBook.insert(ip, Default::default() );
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
     pub fn addWithPorts(&mut self, ip: String, min: u16, max: u16 )
     {
          let mut ss = SplaySet::<u16>::new();


          for x in min..max
          {
              ss.insert(x);
          }
          let ports = Ports { fullRange: false, portRange: ss };
          self.addressBook.insert(ip, ports );

     }
     #[allow(dead_code)]
     pub fn getPorts(&mut self, ip: String) -> &Ports
     {

          return (self.addressBook.get(&ip)).unwrap();
     }
     #[allow(dead_code)]
     pub fn addPort(&mut self, ip: String, port: u16)
     {
          let  ports = (self.addressBook.get_mut(&ip)).unwrap();
          ports.insert(port);
          let y = ports.clone();
          self.addressBook.insert(ip, y);
     }
     #[allow(dead_code)]
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
      #[allow(dead_code)]
     pub fn addData(&mut self, message: String)
     {
          self.info.add(message);
          return;
     }
     #[allow(dead_code)]
     pub fn getData(&mut self) -> String
     {
          return self.info.get();
     }
     #[allow(dead_code)]
     pub fn getListed(&mut self) -> Vec<String>
     {
          let mut vec = Vec::new();
          for key in self.addressBook.keys() {
               vec.push(key.to_string());
           }
           return vec;
     }
     #[allow(dead_code)]
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
     #[allow(dead_code)]
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
     #[allow(dead_code)]
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
      #[allow(dead_code)]


     pub fn main(&mut self)
     {
          //set up the socket so we can connect to publishers and subscribers
          let mut full_address = "tcp://".to_string();
          full_address.push_str(&self.ip);
          full_address.push_str(&":");
          full_address.push_str(&self.port.to_string());
          let context = zmq::Context::new();
          let responder = context.socket(zmq::REP).unwrap();
          responder
               .bind( &(full_address) )
               .expect("failed binding socket");

          //get the port that we are bound to
          let _lastEndpoint = match responder.get_last_endpoint()
          {
               Ok(lastEndpoint) => {
                    match lastEndpoint {
                         Ok(lastEndpoint) => lastEndpoint,
                         Err(_e) => String::new(),
                    }
               },
               Err(_e) => "failed".to_string(),
          };

          let mut msg = zmq::Message::new();


          loop
          {
               //read inbound messages

               //Can never return none cause it waits

               responder.recv(&mut msg, 0).unwrap();

               //data as string
               let data = msg.as_str().unwrap();
               let res = serde_json::from_str(data);

               //json deserialized stored inside p value
               let inbound: Message = res.unwrap();
               let ip = inbound.ip.clone();

               //white/black list check for valid credentials
               if self.validAddress(inbound.ip, inbound.port) == false
               {
                    //do nothing if invalid
               }
               else if inbound.messageType == 'D'
               {
                    //add data
                    //use CLASS ADD FUNCTION
                    self.info.add(inbound.message);
                    let m = Message { messageType: 'A', ip: self.ip.to_string(), port: self.port,  message: "".to_string() };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();

                    //record an incoming message
                    self.channelStatistics.numReceived += 1;
                    self.channelStatistics.numStored = self.info.info.len() as u32;

                    //save timestamp of last message sent for that publisher
                    let start = SystemTime::now();
                    let since_the_epoch = start
                         .duration_since(UNIX_EPOCH)
                         .expect("Time went backwards");
                    let in_ms = since_the_epoch.as_millis();
                    let key = format!("{}:{}", ip, inbound.port);
                    if self.channelStatistics.pubTimestamps.contains_key(&key)
                    {
                         let timestamp = self.channelStatistics.pubTimestamps.get_mut(&key).unwrap();
                         *timestamp = in_ms;
                    }
                    else
                    {
                         self.channelStatistics.pubTimestamps.insert(key, in_ms);
                    }
               }
               else if  inbound.messageType == 'R'
               {
                    //send data
                    let mut temp = "".to_string();

                    if self.styles == "allFIFO"
                    {
                         /*
                         let retval = self.info.getBroadcast(inbound.message.parse::<u32>().unwrap());
                         let x = messaging::PositionText{ position: retval.0, text: retval.1 };
                         let res = serde_json::to_string(&m);
                         temp = res.unwrap();
                         */
                    }
                    else
                    {
                         temp = self.info.get();
                    }
                    let m = Message { messageType: 'D', ip: self.ip.to_string(), port: self.port,  message: temp.to_string() };

                    let res = serde_json::to_string(&m);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();

                    //record an outgoing message - only counts one for each subscriber request
                    self.channelStatistics.numSent += 1;
                    
                    //save timestamp of the last request from that subscriber
                    let start = SystemTime::now();
                    let since_the_epoch = start
                         .duration_since(UNIX_EPOCH)
                         .expect("Time went backwards");
                    let in_ms = since_the_epoch.as_millis();
                    let key = format!("{}:{}", ip, inbound.port);
                    if self.channelStatistics.subTimestamps.contains_key(&key)
                    {
                         let timestamp = self.channelStatistics.subTimestamps.get_mut(&key).unwrap();
                         *timestamp = in_ms;
                    }
                    else
                    {
                         self.channelStatistics.subTimestamps.insert(key, in_ms);
                    }
               }
               else if  inbound.messageType == 'S'
               {
                    //send status
                    /*let temp = String::from("STATUS REQUEST: Not Available");
                    let m = Message { messageType: 'S', ip: self.ip.to_string(), port: self.port,  message: temp }; */
                    
                    //serialize the channel statistics struct and send that back to the master process
                    let res = serde_json::to_string(&self.channelStatistics);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
               }
               else if inbound.messageType == 'T'
               {
                    //terminate channel listening and return to caller
                    let m = Message { messageType: 'A', ip: self.ip.to_string(), port: self.port,  message: "".to_string() };

                    let res = serde_json::to_string(&m);
                    //let res = serde_json::to_string(&self.status);
                    let serial_message: String = res.unwrap();
                    responder.send(&serial_message, 0).unwrap();
                    //println!("channel closed");
                    return;
               }
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
          pub limit: u32,
          pub _deleteOnPull: bool,
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
          #[allow(dead_code)]
          pub fn setPull(&mut self, value: bool)
          {
               self._deleteOnPull = value;
          }
          #[allow(dead_code)]
          pub fn add(&mut self, bytes: String)
          {
               if (self.info.len() as u32) == self.limit
               {
                    self.info.pop_front();
               }
               self.info.push_back(bytes);
          }
          #[allow(dead_code)]
          pub fn setLimit(&mut self, lim: u32)
          {
               self.limit = lim;
          }
          /**
          * get a string to the fifo structure
          *
          * param none
          *
          * return (String) - All data from FIFO
          */
          #[allow(dead_code)]
          pub fn get(&mut self) -> String
          {
               
               //let mut retval = String::from("");
               /*
               for i in &self.info
               {
                    retval.push_str(i);
                    //retval = [retval, i].concat();
               }
               self.info.clear();

               retval.push_str((&self.info.pop_front()).unwrap());
               return retval;
                              */

              let x = self.info.pop_front();
              if x.is_some(){
                   let val = x.unwrap();
                   if self._deleteOnPull == false
                   {

                         self.info.push_front(val.to_string());
                   }
                   return val;
              }else
              {
                  return "".to_string();
              }

          }
          /**
          * New call to return new object
          *
          * param none
          *
          * return (Information) - A blank information object
          */
          #[allow(dead_code)]
          pub fn new() -> Information
          {

               return Information { info: VecDeque::new(), limit: 500, _deleteOnPull: true};
          }
     }

}
