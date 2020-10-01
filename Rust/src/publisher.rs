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

//#[derive(Debug)]

type IPPort = (string, u16);        //tuple that holds (IP, Port)

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
pub struct Publisher
{
    pub channelInfo : hashmap<string,IPPort>,
    pub ip: String,     
    pub port:   u16,
}
impl Publisher
{
    //constructor for Publisher Object
    pub fn new(&mut self, MasterIP: String, MasterPort: u16, Name: String, IP: String, Port: u16) -> Publisher
    {
        return Publisher{self.channelInfo.insert("Master", (&MasterIP, &MasterPort)), &IP, &Port}
    }
    //fn for adding a channel / master info to the map being used for data storage
    pub fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(&Name, (&IP, &Port));   //pass in the info about the channel to be stored in the pub
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
        let messageSent = Message{&"P", &self.ip, &self.port, &""};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object
        
        //concatenate "tcp://" "IP" ":" "PORT" together
        
        let mut a = "tcp://".to_string();
        let b = self.channelInfo.get(&"Master").0.to_string();
        let c = ":".to_string();
        let d = self.channelInfo.get(&"Master").1.to_string();
        
        a.push_str(&b + &c + &d);
        
        //connect to the master object
        client.connect(a)

        //send the message that has been serialized to the master
        client.send(&serialMessage,0).unwrap();

        //wait for the response from master so that I can store the message into the message object
        let mut msg = zmq::Message::new();
        client.recv(&msg,0).unwrap();
        
        //deserialize the information
        
        let data = msg.as_str().unwrap();
        let res = serde_json::from_str(data);
        
        let inbound : Message = res.unwrap();
        //add the information to the channelInfo Object

        self.add(&Name, &inbound.ip, &inbound.port);

        }
    }
    //adds ip address to addressbook with default port range 0-max
    pub fn disconnect(&mut self, Name: String)
    {
        //Check if channel is stored in hashmap
        if  self.channelInfo.contains_key(&Name)
        {
        //open a REQ socket and send some kind of info to master saying that you are no longer publishing to channel
        
        //we havent discussed how we want this to be sent to master in the message mode need to do that

        //if it is remove it
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

        let messageSent = Message{&"D", &self.ip, &self.port, &Mess};         // create message object
        let serialMessage = serde_json::to_string(&messageSent).unwrap();   //serialize message object

        //check the hashmap to see if the channel information is stored
        if  self.channelInfo.contains_key(&ChannelName)
        {
            let mut a = "tcp://".to_string();
            let b = self.channelInfo.get(&ChannelName).0.to_string();
            let c = ":".to_string();
            let d = self.channelInfo.get(&ChannelName).1.to_string();
            
            a.push_str(&b + &c + &d);
            //connect to the channel using the message information

                    //connect to the master object
            client.connect(a)
            
            //send the message that has been serialized to the master
            client.send(&serialMessage,0).unwrap();

            //wait for the response
            let mut msg = zmq::Message::new();
            client.recv(&msg,0).unwrap();
        }
        else
        {
        //if the information is not stored then need to request it from master using connect
        //print message to screen or choose to handle it by calling the connect function
        println!("Please connect to the channel first.")

        //then send information
        }

    }
}
