extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::collections::HashMap;
use pyo3::prelude::*;
#[path = "messaging.rs"] mod messaging;

type IPPort = (String, u16);        //tuple that holds (IP, Port)


/**
 * Subscribers recieve data from channels
 * channelInfo (HashMap<String, IPPort>) - Channel names maped to their addresses
 * masterip (String) - IP of Mster Process
 * masterport (u16) - port of Master Process
 * ip (String) - Publisher's IP address
 * port (u16) - Publisher's port
 */
#[pyclass]
#[derive(Clone)]
pub struct Subscriber
{
    pub channelInfo : HashMap<String,IPPort>,
    pub masterip: String,
    pub masterport: u16,
    pub ip: String,
    pub port:   u16,
}

impl Subscriber
{
    /**
     * constructor for Subscriber Object
     * MasterIP (String) - Ip of Master Process
     * MasterPort (u16) - Port of Master Process
     * IP (String) - IP to host Subscriber on
     * Port (u16) - Port to host Subscriber on
     */
    pub fn new(MasterIP: String, MasterPort: u16, IP: String, Port: u16) -> Subscriber
    {
        
        return Subscriber{channelInfo: HashMap::new(), masterip: MasterIP, masterport: MasterPort, ip : IP, port : Port};
    }
    
    /**
     * Adds a new Channel IP and port to the lookup table
     */
    fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(Name, (IP, Port));   //pass in the info about the channel to be stored in the pub
        
    }

}


#[pymethods]
impl Subscriber {

    /**
     * Represents a Subscriber in a string format
     * Returns (String) - Representation of Subscriber
     */
    pub fn to_string(&mut self) -> String
    {
        return format!("Construct Sub: Master({}, {}) Self({}, {})", self.masterip, self.masterport, self.ip, self.port);
    }

    /**
     * Connects a Subscriber to a Channel
     * Name (String) - Channel to connect to
     */
    pub fn connect(&mut self, Name: String)
    {
        //if it is not stored in the list open up a req socket and send a request to master asking for channel info
        if  self.channelInfo.contains_key(&Name) == false
        {
        let message_ = messaging::Message { messageType: 'C', ip: self.ip.to_string(), port: self.port,  message: Name.to_string() };
        let m2 = messaging::send(self.masterip.to_string(), self.masterport, message_);
        //add the information to the channelInfo Object
        self.add(Name, m2.ip, m2.port);
        }
    }
    
    /**
     * Disconnects a Subscriber from a specific Channel
     * Name (String) - Channel to disconnect from
     */
    pub fn disconnect(&mut self, Name: String)
    {
        //Check if channel is stored in hashmap
        if  self.channelInfo.contains_key(&Name)
        {
         // setup the socket for the client
         let context = zmq::Context::new();
         let client = context.socket(zmq::REQ).unwrap();

         //serialize message for transmission
         let messageSent = messaging::Message{messageType: 'D',ip: self.ip.to_string(),port: self.port,message: Name.to_string()};         // create message object
         let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object

         //concatenate "tcp://" "IP" ":" "PORT" together

         let mut a = "tcp://".to_string();
         a.push_str(&self.masterip.to_string());
         a.push_str(&":");
         a.push_str(&self.masterport.to_string());

         //connect to the master object
         assert!(client.connect(&a).is_ok());

         //send the message that has been serialized to the master
         client.send(&serialMessage,0).unwrap();

         //wait for the response from master so that I can store the message into the message object
         let mut msg = zmq::Message::new();
         client.recv(&mut msg,0).unwrap();

        self.channelInfo.remove(&Name);
        }
        else    //If the channel does not exist in the publisher then don't do anything
        {
        }
    }

    /**
     * Recieves data from a Channel
     * ChannelName (String) - Channel to recieve from
     * Returns (String) - Data recieved from the Channel
     */
    pub fn listen(&mut self, ChannelName : String) -> String
    {
        if  self.channelInfo.contains_key(&ChannelName) == false
        {
            self.connect(ChannelName.to_string());
        }

        let chanInfo = self.channelInfo.get_mut(&ChannelName).unwrap();
        let m = messaging::Message { messageType: 'R', ip: self.ip.to_string(), port: self.port,  message: "".to_string() };
        let m2 = messaging::send(chanInfo.0.to_string(), chanInfo.1, m);


        return m2.message;
    }

    /**
     * Gets the IP of the Subscriber
     * Returns (String) - IP of Subscriber
     */
    pub  fn getIP(&mut self)->String
    {
        return self.ip.to_string();
    }

    /**
     * Gets the port of the Subscriber
     * Returns (u16) - Port of Subscriber
     */
    pub  fn getPort(&mut self)->u16
    {
        return self.port;
    }
}

