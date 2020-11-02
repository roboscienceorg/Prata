extern crate serde_json;
extern crate serde;
extern crate serde_derive;

//use std::collections::HashMap;
use std::collections::HashMap;
//use std::thread;
//use std::time::Duration;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value as JsonValue;
use pyo3::prelude::*;
#[path = "messaging.rs"] mod messaging;
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
pub struct Subscriber
{
    pub channelInfo : HashMap<String,IPPort>,
    pub masterip: String,
    pub masterport: u16,
    pub ip: String,
    pub port:   u16,




}
#[pymethods]
impl Subscriber {

    pub fn to_string(&mut self) -> String
    {
        return format!("Construct Sub: Master({}, {}) Self({}, {})", self.masterip, self.masterport, self.ip, self.port);
    }
    //fn for adding a channel / master info to the map being used for data storage
    pub fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(Name, (IP, Port));   //pass in the info about the channel to be stored in the pub
        
    }
    //function for connecting to a channel
    pub fn connect(&mut self, Name: String)
    {
        //if it is not stored in the list open up a req socket and send a request to master asking for channel info
        if  self.channelInfo.contains_key(&Name) == false
        {
        let mx = messaging::Message { messageType: 'C', ip: self.ip.to_string(), port: self.port,  message: Name.to_string() };
        let m2 = messaging::send(self.masterip.to_string(), self.masterport, mx);
        //add the information to the channelInfo Object
        self.add(Name, m2.ip, m2.port);
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
    pub fn listen(&mut self, ChannelName : String) -> String
    {
        if  self.channelInfo.contains_key(&ChannelName) == false
        {
            self.connect(ChannelName.to_string());
        }

        let chanInfo = self.channelInfo.get_mut(&ChannelName).unwrap();
        let m = messaging::Message { messageType: 'R', ip: self.ip.to_string(), port: self.port,  message: "".to_string() };
        let m2 = messaging::send(chanInfo.0.to_string(), chanInfo.1, m);

        //ALL FIFO CHANCE
        /*
        let txtpos = messaging::JsonToTextPosition(m2.message);
        chanInfo.2 = txtpos.position;
        */
        return m2.message;
    }

}

impl Subscriber
{
    //constructor for Publisher Object
    pub fn new(MasterIP: String, MasterPort: u16, IP: String, Port: u16) -> Subscriber
    {
        
        return Subscriber{channelInfo: HashMap::new(), masterip: MasterIP, masterport: MasterPort, ip : IP, port : Port};
    }



}
