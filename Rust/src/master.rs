#[path = "master_process.rs"] mod master_process;
//mod master_process::MasterProcess;
use std::collections::HashMap;
use pyo3::prelude::*;
use serde_json;
use serde;
//use serde_derive;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use port_scanner::request_open_port;
use std::thread;
extern crate ipconfig;

#[path = "subscriber.rs"] mod subscriber;
#[path = "publisher.rs"] mod publisher;

#[pyclass]
#[derive(Clone)]
#[derive(Debug)]
pub struct Master
{
   pub ipAddress: String,
   pub port: u16,
   pub threading: bool,
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
   pub fn serialize(&self) -> String
   {
      let context = zmq::Context::new();
      let responder = context.socket(zmq::REQ).unwrap();

      let protocol = "tcp://".to_string();
      let str1 = String::from(&self.ipAddress);
      let str2 = String::from(":");
      let str_with_port = self.port.to_string();
      let address = [protocol, str1, str2, str_with_port].concat();

      assert!(responder.bind(&address).is_ok());
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

      return json_data.message;
      }

   pub fn setThreading( &mut self, value: bool)
   {
      self.threading = value;
   }


  pub fn subscriber( &self) -> subscriber::Subscriber
   {
      let port = request_open_port().unwrap_or(0);
      let addr = self.getLocalIp().to_string();


      return subscriber::Subscriber::new(self.ipAddress.to_string(), self.port, addr,port);
   }



   /* Return a publisher object */
   pub fn publisher( &self) -> publisher::Publisher
   {
      //just need publisher constructor
      let port = request_open_port().unwrap_or(0);
      let addr = self.getLocalIp().to_string();

      return publisher::Publisher::new(self.ipAddress.to_string(), self.port, addr,port);
   }

   pub fn host(&self)
   {
      let s = self.ipAddress.to_string();
      let p = self.port;
      //let p = self.port;
      if self.threading{
      thread::spawn( move || {
         let mp = master_process::MasterProcess { channels: HashMap::new(), ipAddress: s, port: p  };
         mp.start();
      });
      }
      else
      {
         let mp = master_process::MasterProcess { channels: HashMap::new(), ipAddress: s, port: p  };
         mp.start();

      }


   }
   pub fn terminate(&self)
   {
      let context = zmq::Context::new();
      let responder = context.socket(zmq::REQ).unwrap();

      let protocol = "tcp://".to_string();
      let str1 = String::from(&self.ipAddress);
      let str2 = String::from(":");
      let str_with_port = self.port.to_string();
      let address = [protocol, str1, str2, str_with_port].concat();

      assert!(responder.bind(&address).is_ok());
      let m = Message { messageType: 'T', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };

      let res = serde_json::to_string(&m);
      let serial_message: String = res.unwrap();
      let mut msg = zmq::Message::new();

      responder.send(&serial_message, 0).unwrap();
      responder.recv(&mut msg, 0).unwrap();

   }


}
impl Master
{
   pub fn new() -> Master
   {
      let p = request_open_port().unwrap_or(0);
      let addr = (Ipv4Addr::LOCALHOST).to_string();

      return Master {ipAddress: addr, port: p, threading: true};
   }
   /* Starts a host process in this thread. */


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

   pub fn getLocalIp( &self ) -> String
   {
      let network_info = ipconfig::get_adapters().unwrap();

      let mut ip = "".to_string();
      let start = 127;
      for cards in network_info
      {
          for ips in cards.ip_addresses()
          {
              //println!("{:?}", ips);
              match ips 
              {
                  std::net::IpAddr::V4(value) => 
                      if value.octets()[0] != start
                      {
                          ip = value.to_string();
                      },
                  _ => (),
              }
          }
      }
   

   return ip.to_string();
   }
}
/* Saves the credentials for the remote master process*/
#[allow(dead_code)]
pub fn connect(ip: String, p: u16 ) -> Master
{
   return Master {ipAddress: ip, port: p, threading: true}
}
