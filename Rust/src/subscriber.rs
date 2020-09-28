extern crate serde_json;
extern crate serde;
extern crate serde_derive;


use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;

//#[derive(Debug)]




//structure for messages that are going to be sent
#[derive(Serialize, Deserialize)]
pub struct InboundMessage
{
     pub messageType: char,         //type of message being sent to channel   
     pub ip: u32,                   //IP of channel message is being sent to
     pub port: u16,                 //port of channel message is being sent to
     pub message: String,           //message being sent

}


//structure for all data that publisher needs to transmit data
pub struct Subscriber
{
    //hashmap<string,vector>
     pub name: String,                  //name of channel
     pub port: u16,                     //port for listening
     pub info: self::data::Information, //information storage
     //maps an ip to its port range     
}

impl Subscriber
{
    //function for connecting to a channel
    pub fn connect()
    {
        
    }
    //adds ip address to addressbook with default port range 0-max
    pub fn disconnect()
    {

    }
}