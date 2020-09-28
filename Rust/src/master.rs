mod master_process;
use master_process::MasterProcess;
use std::collections::HashMap;


pub struct Master
{
   pub ipAddress: String,
   pub port: u16,
}

impl Master
{
   /* Starts a host process in this thread. */
   pub fn host(self)
   {
      let mp = MasterProcess { channels: HashMap::new(), ipAddress: self.ipAddress, port: self.port };
      mp.start();
   }

   /* Connect to a remote master process
      Return a credential object */
   pub fn connect(ip: String)
   {
      //is this necessary?
   }

   /* Launch gui for current master 
      Return error if no master */
   pub fn gui()
   {
      //gui launch
   }

   /* Disconnect from master */
   pub fn disconnect()
   {

   }

   /* Return a subscriber object */
   pub fn subscriber()
   {
      //just need subscriber constructor
   }

   /* Return a publisher object */
   pub fn publisher()
   {
      //just need publisher constructor
   }
   
}


