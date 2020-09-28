extern crate serde_json;
extern crate serde;
extern crate serde_derive;

//use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;

//#[derive(Debug)]




//structure for messages that are going to be sent
#[derive(Serialize, Deserialize)]
pub struct OutboundMessage
{
     pub messageType: char,         //type of message being sent to channel   
     pub ip: u32,                   //IP of channel message is being sent to
     pub port: u16,                 //port of channel message is being sent to
     pub message: String,           //message being sent

}


//structure for all data that publisher needs to transmit data
pub struct Publisher
{
    //hashmap<string,vector>
     pub name: String,                  //name of channel
     pub port: u16,                     //port for listening
     pub info: self::data::Information, //information storage
     //maps an ip to its port range     
}
impl Publisher
{
    //function for connecting to a channel
    pub fn connect(/*name of channel to connect to*/)
    {
        //check the currently stored channels to see if it is stored there

        //if it isn't send it master to get the address of the channel of it exists
        //not sure how it is going to be sent back or if I will need a zmq subscriber to receive the information
    }
    //adds ip address to addressbook with default port range 0-max
    pub fn disconnect(/*name of channel to disconnect from*/)
    {
        //Check if channel is stored in hashmap

        //if it is remove it
    }
    pub fn publish(/*channel name, data being sent*/)
    {
        //check the hashmap to see if the channel information is stored
        
        //if the information is not stored then need to request it from mast
        //not 100% sure how this is being stored
    }
}
