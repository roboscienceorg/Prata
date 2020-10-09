extern crate serde_json;
extern crate serde;
extern crate serde_derive;
//use std::time::Duration;
//use std::collections::HashMap;
use std::collections::HashMap;
//use std::thread;
//use std::time::Duration;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value as JsonValue;
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
    pub fn new(MasterIP: String, MasterPort: u16, IP: String, Port: u16) -> Publisher
    {
        println!("Construct Sub: Master({}, {}) Self({}, {})", MasterIP, MasterPort, IP, Port);
        return Publisher{channelInfo: HashMap::new(), masterip: MasterIP, masterport: MasterPort, ip : IP, port : Port}
    }
    //fn for adding a channel info to the map being used for data storage
    pub fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(Name, (IP, Port));   //pass in the info about the channel to be stored in the pub
    }
    //function for connecting to a channel

}

#[pymethods]
impl Publisher{
    pub fn connect(&mut self, Name: String)
    {
        //check the currently stored channels to see if it is stored there
        if  self.channelInfo.contains_key(&Name)
        {
            return;
        }
        //if it is not stored in the list open up a req socket and send a request to master asking for channel info
        else
        {

        let context = zmq::Context::new();
        let responder = context.socket(zmq::REQ).unwrap();

        let protocol = "tcp://".to_string();
        let str1 = String::from(&self.masterip);
        let str2 = String::from(":");
        let str_with_port = self.masterport.to_string();
        let address = [protocol, str1, str2, str_with_port].concat();

        assert!(responder.bind(&address).is_ok());
        let m = Message { messageType: 'c', ip: self.ip.to_string(), port: self.port,  message: Name.to_string() };

        let res = serde_json::to_string(&m);
        let serial_message: String = res.unwrap();
        let mut msg = zmq::Message::new();

        responder.send(&serial_message, 0).unwrap();
        responder.recv(&mut msg, 0).unwrap();


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
        let messageSent = Message{messageType: 'd',ip: self.ip.to_string(),port: self.port,message: Name.to_string()};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object

        //concatenate "tcp://" "IP" ":" "PORT" together

        let mut a = "tcp://".to_string();
        a.push_str(&self.masterip.to_string());
        a.push_str(&":");
        a.push_str(&self.masterport.to_string());

         //connect to the master object
        assert!(client.bind(&a).is_ok());

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


        if  self.channelInfo.contains_key(&ChannelName) == false
        {
            self.connect(ChannelName.to_string());
        }


        let chanInfo = self.channelInfo.get(&ChannelName).unwrap();

        let chanIP = &chanInfo.0;
        let chanPort = &chanInfo.1;

        let context = zmq::Context::new();
        let responder = context.socket(zmq::REQ).unwrap();

        let protocol = "tcp://".to_string();
        let str1 = String::from(chanIP.to_string());
        let str2 = String::from(":");
        let str_with_port = chanPort.to_string();
        let address = [protocol, str1, str2, str_with_port].concat();

        assert!(responder.bind(&address).is_ok());
        let m = Message { messageType: 'D', ip: self.ip.to_string(), port: self.port,  message: Mess.to_string() };

        let res = serde_json::to_string(&m);

        let serial_message: String = res.unwrap();
        let mut msg = zmq::Message::new();


        responder.send(&serial_message, 0).unwrap();
        responder.recv(&mut msg, 0).unwrap();
    }
}
