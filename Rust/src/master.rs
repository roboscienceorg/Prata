#[path = "master_process.rs"] mod master_process;
//mod master_process::MasterProcess;
use std::collections::HashMap;
use pyo3::prelude::*;
use serde_json;
use serde;
use serde_derive;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use port_scanner::request_open_port;

#[path = "subscriber.rs"] mod subscriber;
#[path = "publisher.rs"] mod publisher;

#[pyclass]
#[derive(Clone)]
#[derive(Debug)]
pub struct Master
{
   pub ipAddress: String,
   pub port: u16,
}

/* universal message format */
#[derive(Serialize, Deserialize)] 
pub struct Message 
{     
   pub messageType: char,            
   pub ip: String,  
   pub port: u16, 
   pub message: String, 
} 

#[pymethods]
impl Master
{
   pub fn serialize(&self) -> PyResult<String>
   {
      let context = zmq::Context::new();
      let responder = context.socket(zmq::REQ).unwrap();

      let protocol = "tcp".to_string();
      let str1 = String::from("://*:");
      let str_with_port = self.port.to_string();
      let address = [protocol, str1, str_with_port].concat();


      let m = Message { messageType: 'J', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };

      let res = serde_json::to_string(&m);
      let serial_message: String = res.unwrap();
      let mut msg = zmq::Message::new();
      
      responder.send(&serial_message, 0).unwrap();
      responder.recv(&mut msg, 0).unwrap();
      //data as string
      let data = msg.as_str().unwrap();
      let res = serde_json::from_str(data);
      //json deserialized stored inside p value
      let json_data: Message = res.unwrap();
      Ok(json_data.message)


   }
}
impl Master
{
   pub fn new() -> Master
   {
      let p = request_open_port().unwrap_or(0);
      let addr = (Ipv4Addr::LOCALHOST).to_string();
      
      return Master {ipAddress: addr, port: p};
   }
   /* Starts a host process in this thread. */
   pub fn host(&self)
   {

      let mp = master_process::MasterProcess { channels: HashMap::new(), ipAddress: self.ipAddress.to_string(), port: self.port };
      mp.start();

   }

   /* Saves the credentials for the remote master process*/
   pub fn connect( mut self, ip: String, port: u16 )
   {
      self.port = port;
      self.ipAddress = ip;
   }

   /* Launch gui for current master 
      Return error if no master */
   pub fn gui( self)
   {
      //gui launch
   }

   /* Disconnect from master */
   pub fn disconnect( &self)
   {

   }

   /* Return a subscriber object */
   pub fn subscriber( &self) -> subscriber::Subscriber
   {
      //just need subscriber constructor
      let port = request_open_port().unwrap_or(0);
      //let octets = (Ipv4Addr::LOCALHOST).octets();
      let addr = (Ipv4Addr::LOCALHOST).to_string();
      println!("your subscriber ip is {}",addr);
/*
      let mut addr = String::from("");
      for i in &octets
      {
           addr.push_str(i.to_string());
           addr.push_str(".".to_string());
      }
      let len = addr.len();
      addr.truncate(len - 1);
      self.info.clear();
      return retval;
*/

      return subscriber::Subscriber::new(self.ipAddress.to_string(), self.port, addr,port);
   }

   /* Return a publisher object */
   pub fn publisher( &self) -> publisher::Publisher
   {
      //just need publisher constructor
      let port = request_open_port().unwrap_or(0);
      let addr = (Ipv4Addr::LOCALHOST).to_string();
      println!("your publisher ip is {}",addr);

      return publisher::Publisher::new(self.ipAddress.to_string(), self.port, addr,port);
   }


   
}

