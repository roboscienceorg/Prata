


//use std::collections::Ve
use splay::SplayMap;
use splay::SplaySet;


#[derive(Debug)]


pub enum ChannelMode
{
    STANDARD,
    WHITELIST,
    BLACKLIST
}
impl Default for ChannelMode
{
     fn default() -> Self {ChannelMode::STANDARD}
}

/*
pub struct MinMax
{
    pub min: u16,
    pub max: u16,
}
impl Default for MinMax
{
    fn default() -> MinMax 
    {
        //sets the min to min and max to max
        MinMax { min: u16::MIN, max: u16::MAX}
    }

}
*/
pub struct Ports
{
     pub fullRange: bool,
     pub portRange: SplaySet<u16>,
}
impl Default for Ports
{
     fn default() -> Ports 
     {
          Ports
          {
               fullRange: true,
               portRange: SplaySet::new()
          }
     }
}


pub struct Channel
{
     pub mode: ChannelMode,
     pub name: String,
     //publisher pub sender: ZMQ::Publisher()
     //subscriber pub reciever: ZMQ::Subscriber()
     pub info: self::data::Information,
     //maps an ip to its port range
     pub addressBook : SplayMap<i32,Ports>,
     


}
impl Default for Channel
{
    fn default() -> Channel 
    {
        Channel
        {
            mode: ChannelMode::STANDARD,
            name: String::from("1"),
            info: self::data::Information::new(),
            addressBook: SplayMap::new()
        }
    }
}
impl ToString for Channel
{
     fn to_string(&self) -> String
     {
        return format!("Name:{} Mode:{:?}",self.name, self.mode);
     }
}
impl Channel
{
    //adds ip address to addressbook with default port range 0-max
    pub fn add(&mut self, ip: i32 )
    {
        self.addressBook.insert(ip, Default::default() );
    }

     //adds ip address to addressbook with inclusvie port range min-max
     pub fn addWithPorts(&mut self, ip: i32, min: u16, max: u16 )
     {
          let mut ss: SplaySet<u16>;
 

          for x in min..max
          {
               //&ss.clear();
          }
          let mut ports = Ports { fullRange: false, ..Default::default() };
          self.addressBook.insert(ip, ports );
     
     }


}




mod data
{
     use std::collections::VecDeque;

     pub struct Information
     {
          pub info: self::VecDeque<String>,
     }


     impl Information
     {
          pub fn add(&mut self, bytes: String)
          {
               self.info.push_back(bytes);
          }
     }
     impl Information
     {
          pub fn new() -> Information
          {
               return Information { info: VecDeque::new()};
          }
     }
}