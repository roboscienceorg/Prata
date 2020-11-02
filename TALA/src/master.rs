#[path = "master_process.rs"] mod master_process;
#[path = "messaging.rs"] mod messaging;
#[path = "channel.rs"] mod channel;
#[path = "subscriber.rs"] mod subscriber;
#[path = "publisher.rs"] mod publisher;

use std::collections::HashMap;
use pyo3::prelude::*;
use serde_json;
use std::net::Ipv4Addr;
use port_scanner::request_open_port;
use std::thread;

extern crate get_if_addrs;
extern crate local_ipaddress;


/**
 * Struct that defines the properties for the Master object.
 * ipAddress (String) - ip address the master object is available at
 * port (u32) - port the master object is available at
 * threading (bool) - flag that sets whether or not to use multithreading for creating channels.
 */
#[pyclass]
#[derive(Clone)]
#[derive(Debug)]
pub struct Master
{
   pub ipAddress: String,
   pub port: u16,
   pub threading: bool,
}

/**
 * Defines the functions for the master object.
 */
#[pymethods]
impl Master
{
   /**
   * Requests a json serialized representation of the master process, 
   * receives it, and returns it.
   */
   pub fn serialize(&self) -> String
   {
      let m = messaging::Message { messageType: 'J', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string() };
      let m2 = messaging::send(self.ipAddress.to_string(), self.port, m);
      return m2.message;
   }

   /**
   * Requests master process to set port ranges.
   */
   pub fn setPortRanges(&self, lower: u16, upper: u16)
   {
      let m = messaging::Message { messageType: 'P', ip: self.ipAddress.to_string(), port: self.port,  message: [lower.to_string(), upper.to_string()].join(":") };
      messaging::send(self.ipAddress.to_string(), self.port, m);
   }
   
   /**
   * Requests master process to create a new custom channel.
   * port (u16) - port to create the channel at
   * name (String) - name of the new channel
   * style (String) - FIFO or Broadcast I think
   * messageLimit (u32) - sets the number of messages the channel can store at one time
   */
   pub fn createChannel(&self, port_: u16, name_: String, style_: String, messageLimit_: u32)
   {
      let config = channel::ChannelConfiguration::new(self.ipAddress.to_string(), port_, name_.to_string(), style_.to_string(), messageLimit_);
      let res = serde_json::to_string(&config);
      let serial_message: String = res.unwrap();

      let m = messaging::Message { messageType: 'N', ip: self.ipAddress.to_string(), port: self.port,  message: serial_message.to_string() };
      messaging::send(self.ipAddress.to_string(), self.port, m);

   }

   /**
   * Returns the supported channel types
   */
   pub fn getChannelTypes(&self)
   {
      let m = messaging::Message { messageType: 'O', ip: self.ipAddress.to_string(), port: self.port,  message: "".to_string()};
      messaging::send(self.ipAddress.to_string(), self.port, m);
   }

   /**
   * Kills a channel process and removes its' data from master process.
   * name (String) - name of the channel to be destroyed.
   */
   pub fn removeChannel(&self, name: String)
   {
      let m = messaging::Message { messageType: 'R', ip: self.ipAddress.to_string(), port: self.port,  message: name.to_string() };
      messaging::send(self.ipAddress.to_string(), self.port, m);
   }

   /**
   * Sets the boolean flag for whether or not to use multithreading for creating master process.
   * value (bool) - True to enable multithreading, False for no multithreading.
   */
   pub fn setThreading( &mut self, value: bool)
   {
      self.threading = value;
   }

   /**
   * Returns a new subscriber object
   */
  pub fn subscriber( &self) -> subscriber::Subscriber
   {
      let port = request_open_port().unwrap_or(0);
      let addr = self.getLocalIp().to_string();


      return subscriber::Subscriber::new(self.ipAddress.to_string(), self.port, addr,port);
   }



   /* 
   * Returns a new publisher object 
   */
   pub fn publisher( &self) -> publisher::Publisher
   {
      let port = request_open_port().unwrap_or(0);
      let addr = self.getLocalIp().to_string();
      return publisher::Publisher::new(self.ipAddress.to_string(), self.port, addr,port);
   }

   /* 
   * If multithreading is enables, creates a new master process on a separate thread.
   * If multithreading is not enabled, creates and hosts a new master process on this thread.
   */
   pub fn host(&self)
   {

      let s = self.ipAddress.to_string();
      let p = self.port;
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
   /* 
   * Returns a new master object.
   */
   pub fn new() -> Master
   {
      let p = request_open_port().unwrap_or(0);
      let addr = (Ipv4Addr::LOCALHOST).to_string();
      return Master {ipAddress: addr, port: p, threading: true};
   }

   /* 
   * Gets the local ip address of this process/device.
   */
   pub fn getLocalIp( &self ) -> String
   {
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

/* 
* Saves credentials for Master and returns a new Master object.
*/
#[allow(dead_code)]
pub fn connect(ip: String, p: u16 ) -> Master
{
   return Master {ipAddress: ip, port: p, threading: true}
}
