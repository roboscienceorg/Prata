#[path = "master_process.rs"] mod master_process;
//mod master_process::MasterProcess;
use std::collections::HashMap;
use pyo3::prelude::*;



#[pyclass]
#[derive(Clone)]
pub struct Master
{
   pub ipAddress: String,
   pub port: u16,
}


impl Master
{
   pub fn new() -> Master
   {
      return Master {ipAddress: "192.test".to_string(), port: 25565};
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
   pub fn subscriber( self)
   {
      //just need subscriber constructor
   }

   /* Return a publisher object */
   pub fn publisher( self)
   {
      //just need publisher constructor
   }
   
}

