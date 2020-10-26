#[path = "master_process.rs"] mod master_process;
#[path = "messaging.rs"] mod messaging;
#[path = "channel.rs"] mod channel;
//mod master_process::MasterProcess;
use std::collections::HashMap;
use pyo3::prelude::*;
use serde_json;
use serde;
//use serde_derive;
//use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use port_scanner::request_open_port;
use std::thread;
extern crate get_if_addrs;
extern crate local_ipaddress;
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



#[pymethods]
impl Master
{
   pub fn serialize(&self) -> String
   {

      let m = messaging::Message { messageType: 'J', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
      let m2 = messaging::send(self.ipAddress.to_string(), self.port, m);
      return m2.message;

   }

   pub fn setPortRanges(&self, lower: u16, upper: u16)
   {
      let m = messaging::Message { messageType: 'P', ip: self.ipAddress.to_string(), port: self.port,  message: [lower.to_string(), upper.to_string()].join(":") };
      messaging::send(self.ipAddress.to_string(), self.port, m);
   }
   
   pub fn createChannel(&self, port_: u16, name_: String, style_: String, messageLimit_: u32)
   {
      let config = channel::ChannelConfiguration::new(self.ipAddress.to_string(), port_, name_.to_string(), style_.to_string(), messageLimit_);
      let res = serde_json::to_string(&config);
      let serial_message: String = res.unwrap();

      let m = messaging::Message { messageType: 'N', ip: self.ipAddress.to_string(), port: self.port,  message: serial_message.to_string() };
      messaging::send(self.ipAddress.to_string(), self.port, m);

   }

   pub fn getChannelTypes(&self)
   {
      let m = messaging::Message { messageType: 'O', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string()};
      messaging::send(self.ipAddress.to_string(), self.port, m);
      //println!("{:?}", recieve.message);
   }

   pub fn removeChannel(&self, name: String)
   {
      let m = messaging::Message { messageType: 'R', ip: self.ipAddress.to_string(), port: self.port,  message: name.to_string() };
      messaging::send(self.ipAddress.to_string(), self.port, m);
      //println!("back from send in master");
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
         let mp = master_process::MasterProcess { channels: HashMap::new(), ipAddress: s, port: p, nextPort: 0, portRange: (0,0), isCustomRange: false };
         mp.start();
      });
      }
      else
      {
         let mp = master_process::MasterProcess { channels: HashMap::new(), ipAddress: s, port: p, nextPort: 0, portRange: (0,0), isCustomRange: false };
         mp.start();

      }


   }
   pub fn terminate(&self)
   {
      let m = messaging::Message { messageType: 'T', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string()};
      messaging::send(self.ipAddress.to_string(), self.port, m);
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



   /* Return a subscriber object */

   pub fn getLocalIp( &self ) -> String
   {
      //let network_info = ipconfig::get_adapters().unwrap();
      // println!("{:?}", network_info);
      #[allow(unused_assignments)]
      let mut ip = "".to_string();
      if self.ipAddress == "127.0.0.1"{
          ip = "127.0.0.1".to_string()
      }else
      {
         ip = local_ipaddress::get().unwrap().to_string();
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
