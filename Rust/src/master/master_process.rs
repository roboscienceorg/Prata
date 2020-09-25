use std::collections::HashMap;
use zmq::Socket;
use std::clone::Clone;
use port_scanner::request_open_port;
use std::thread;
use std::time::Duration;

/* tuple to hold the address as a string, port as a number */
type AddressPort = (String, u16);

/* struct to hold channel information */
#[derive(Clone)]
pub struct ChannelInfo
{
   name: String,
   info: AddressPort,
   publishers: Vec<AddressPort>,
   subscribers: Vec<AddressPort>,
}

pub struct MasterProcess
{
   //hash by channel name, store channel objects
   pub channels: HashMap<String, ChannelInfo>,
   pub ipAddress: String,
   pub port: u16,
}

impl MasterProcess
{
   /* Start the master process. This will be the main loop */
   pub fn start(mut self)
   {
      //set up the socket so we can connect to publishers and subscribers
      let context = zmq::Context::new();
      let subSocket = context.socket(zmq::SUB).unwrap();
      subSocket
         .connect("tcp://0.0.0.0:7000")
         .expect("failed connecting subscriber");
      subSocket
         .set_subscribe(b"H")
         .expect("failed setting subscription");
      println!("subSocket connected and subscribed");
      thread::sleep(Duration::from_millis(1));

      //get the port that we are bound to
      let lastEndpoint = match subSocket.get_last_endpoint()
      {
         Ok(lastEndpoint) => {
            match lastEndpoint {
               Ok(lastEndpoint) => lastEndpoint,
               Err(_e) => String::new(),
            }
         },
         Err(_e) => "failed".to_string(),
      };

      println!("{}", lastEndpoint);

      //start main loop
      loop 
      {
         //wait for a message to come in from a subscriber or publisher
         let msg_bytes = match subSocket.recv_bytes(0) 
         {
            Ok(msg_bytes) => msg_bytes,
            Err(_e) => Vec::new(),
         }; 

         //the message has to be at least 9 bytes, assuming that we don't allow empty channel names
         if msg_bytes.len() > 8 
         {
         
            //parse the message
            let mut mode = 0;
            let mut channelName: String = "".to_string();
            let mut nodeIP: String = "".to_string();
            let mut port: u16 = 0;
            MasterProcess::parseMessage(&msg_bytes, &mut nodeIP, &mut port, &mut mode, &mut channelName);

            println!("Mode: {}", mode);
            println!("IP: {}", nodeIP);
            println!("Port: {}", port);
            println!("Channel Name: {}", channelName); 
   
            //if the channel doesn't already exist, create it
            if self.channels.contains_key(&channelName) == false
            {
               self.channels.insert(channelName.clone(), MasterProcess::newChannel( self.ipAddress.clone(), channelName.clone() )) ;
            }
   
            //get the channel information
            let channelInfo = &self.channels[&channelName].clone();
   
            //convert the ip address of the channel to a byte array
            let channelIP = &channelInfo.info.0.as_bytes();
            let channelPort = &channelInfo.info.1.to_be_bytes();
          
            let mut returnMessage: Vec<u8> = Vec::new();
            let ipLen = channelIP.len();
   
            for i in 0..( channelIP.len() + 2 )
            {
               if i < ipLen
               {
                  returnMessage.push(channelIP[i]);
               }
               else
               {
                  returnMessage.push(channelPort[i - ipLen]);
               }
            }
   
            //bind to temporary socket on the node and send the data back
            //can't really do this with publish because we only want it to go to one node
           /* let returnSocket = context.socket(zmq::PAIR).unwrap(); 
            returnSocket.connect( &("tcp://".to_owned() + &nodeIP + ":" + &port.to_string()) );
            returnSocket.send(returnMessage,0); */
   
            /* if we want to exit, call break; */
         }
         else
         {
            println!("Received invalid message...");
         }
      }; 
         

      
   }

   /********************** PRIVATE ******************/

   /* Creates a new channel process */
   fn newChannel(ipAddress: String, channelName: String) -> ChannelInfo
   {

      /* TODO: get the port used for this channel, already know ip because its the same machine  */
      let port = request_open_port().unwrap_or(0);
      thread::spawn(move || {
         let mut c = channel::Channel { mode: channel::ChannelMode::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
      });

      let contactInfo: AddressPort = ( ipAddress, port );
      let newChann = ChannelInfo { name: channelName, info: contactInfo, publishers: Vec::new(), subscribers: Vec::new(), };
      return newChann;
   }

   /* takes in a message from a pub/sub and decodes it */
   fn parseMessage(byte_msg: &Vec<u8>, ipString: &mut String, port: &mut u16, mode: &mut u8, channelName: &mut String)
   {
      //first 4 bytes are the sender ip address, so lets extract that
      *ipString = [byte_msg[0].to_string(),
                   byte_msg[1].to_string(),
                   byte_msg[2].to_string(), 
                   byte_msg[3].to_string()]
                   .join(".");

      //next two bytes are the port
      *port = ((byte_msg[4] as u16) << 8) | byte_msg[5] as u16;

      //next byte is the mode
      //0 for publisher, 1 for subscriber, 0 for node ping
      *mode = byte_msg[6];

      //next byte is reserved, do nothing

      //the rest of the bytes are the channel name, so just go until the byte vector has ended
      *channelName = String::from_utf8( byte_msg[7..].to_vec() ).unwrap();

   }


}

/* example usage for parseMessage 

let v: Vec<u8> = vec![1, 2, 3, 4, 0, 4, 10, 0, 240, 159, 146, 150];
let mut mode = 0;
let mut channelName: String = "".to_string();
let mut ipString: String = "".to_string();
let mut port: u16 = 0;
MasterProcess::parseMessage(&v, &mut ipString, &mut port, &mut mode, &mut channelName);
println!("Mode: {}", mode);
println!("IP: {}", ipString);
println!("Port: {}", port);
println!("Channel Name: {}", channelName);

*/