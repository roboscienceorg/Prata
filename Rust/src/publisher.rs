extern crate serde_json;
extern crate serde;
extern crate serde_derive;

//use std::collections::HashMap;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;
use pyo3::prelude::*;

//#[derive(Debug)]

type IPPort = (String, u16);        //tuple that holds (IP, Port)

//structure for messages that are going to be sent
#[derive(Serialize, Deserialize)] 
pub struct Message 
{     
     pub messageType: char,
   
     pub ip: String,  
     pub port: u16, 
     pub message: String, 
} 

//structure for all data that publisher needs to transmit data
#[pyclass]
#[derive(Clone)]
pub struct Publisher
{
    pub channelInfo : HashMap<String,IPPort>,
    pub masterip: String,
    pub masterport: u16,       
    pub ip: String,     
    pub port:   u16,
}
impl Publisher
{
    //constructor for Publisher Object
    pub fn new(&mut self, MasterIP: String, MasterPort: u16, IP: String, Port: u16) -> Publisher
    {
        return Publisher{channelInfo: HashMap::new(), masterip: MasterIP, masterport: MasterPort, ip : IP, port : Port}
    }
    //fn for adding a channel info to the map being used for data storage
    pub fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(Name, (IP, Port));   //pass in the info about the channel to be stored in the pub
    }
    //function for connecting to a channel
    pub fn connect(&mut self, Name: String)
    {
        //check the currently stored channels to see if it is stored there
        if  self.channelInfo.contains_key(&Name)
        {
        }
        //if it is not stored in the list open up a req socket and send a request to master asking for channel info
        else
        {
         // setup the socket for the client
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ).unwrap();
        
        //serialize message for transmission
        let messageSent = Message{messageType: 'P',ip: self.ip.to_string(),port: self.port,message: "".to_string()};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object
        
        //concatenate "tcp://" "IP" ":" "PORT" together
        
        let mut a = "tcp://".to_string();
        a.push_str(&self.masterip.to_string());
        a.push_str(&":");
        a.push_str(&self.masterport.to_string());
                
        //connect to the master object
        client.connect(&a);

        //send the message that has been serialized to the master
        client.send(&serialMessage,0).unwrap();

        //wait for the response from master so that I can store the message into the message object
        let mut msg = zmq::Message::new();
        client.recv(&mut msg,0).unwrap();
        
        //deserialize the information
        
        let data = msg.as_str().unwrap();
        let res = serde_json::from_str(data);
        
        let inbound : Message = res.unwrap();
        //add the information to the channelInfo Object

        self.add(Name, inbound.ip, inbound.port);
        }
    }
    //adds ip address to addressbook with default port range 0-max
    pub fn disconnect(&mut self, Name: String)
    {
        //Check if channel is stored in hashmap
        if  self.channelInfo.contains_key(&Name)
        {
        // setup the socket for the client
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ).unwrap();
         
        //serialize message for transmission
        let messageSent = Message{messageType: 'D',ip: self.ip.to_string(),port: self.port,message: Name.to_string()};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object
         
        //concatenate "tcp://" "IP" ":" "PORT" together
         
        let mut a = "tcp://".to_string();
        a.push_str(&self.masterip.to_string());
        a.push_str(&":");
        a.push_str(&self.masterport.to_string());
                 
         //connect to the master object
        client.connect(&a);
 
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
    pub fn publish(&mut self, ChannelName : String, Mess: String)
    {
        let context = zmq::Context::new();
        let client = context.socket(zmq::REQ).unwrap();

        let messageSent = Message{messageType: 'D',ip: self.ip.to_string(),port: self.port,message: Mess};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object

        //check the hashmap to see if the channel information is stored
        if  self.channelInfo.contains_key(&ChannelName)
        {
        }
        else
        {
        //if the information is not stored then need to request it from master using connect
        //print message to screen or choose to handle it by calling the connect function
        self.connect(ChannelName.to_string());
        }
                
        let mut a = "tcp://".to_string();
        let b = self.channelInfo.get(&ChannelName).unwrap().0.to_string();   //ip   doesnt handle the none case and might cause probs
        let c = ":".to_string();
        let d = self.channelInfo.get(&ChannelName).unwrap().1.to_string();   //port doesnt handle the none case and might cause probs
            

        a.push_str(&b);
        a.push_str(&c);
        a.push_str(&d);
        //connect to the channel using the message information

        //connect to the channel object
        client.connect(&a);
            
        //send the message that has been serialized to the master
        client.send(&serialMessage,0).unwrap();

        //wait for the response
        let mut msg = zmq::Message::new();
        client.recv(&mut msg,0).unwrap();
    }
}
