#![allow(non_snake_case)]
#[allow(dead_code)]
extern crate pyo3;          //used to create python hooks
extern crate zmq;
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::net::Ipv4Addr;
mod channel;
mod master;
mod publisher;
mod subscriber;
use pyo3::prelude::*;
use master::Master;
use publisher::Publisher;
use subscriber::Subscriber;


/**
 * This is the Python prata module
 * This function will set the version of the library
 * It will make connect a public accessible function. This function mimics
 * calling Master.connect()
 * It also allows public access to Master, Subscriber, and Publisher objects
 * 
 * Returns (PyResult) - Python Error, in this case no error
 */
#[pymodule]
fn prata(_py: Python, m: &PyModule) -> PyResult<()> {

    //Set version data for library
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    //Create they python hook for Connect
    #[pyfn(m, "connect")]
    fn connect(py: Python, IP: String, p: u16) -> Py<Master>{

        Py::new(py, Master{ipAddress: IP, port: p, threading: true}).unwrap()
    }

    //Add accessibility to classes
    m.add_class::<Master>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Publisher>()?;

    Ok(())
}



use std::ffi::{CString, CStr};
use std::os::raw::c_char;

/**
 * What is below are external library access points
 * When compiled to a library, these functions are publicly accessible
 * to the linker of the compiler. The functions are prefixed with no_mangle
 * so their names are preserved and keywords in the library namespace.
 * 
 * These functions are used to allow Julia to interface with the Prata library.
 * These functions could also be called by any language that can include a dll or so
 * file into it's project.
 * 
 * Variable returned by these functions are placed on the heap and might cause 
 * memory leaks(not tested). If the variables are not on the heap they will be
 * deconstructed when the function returns and will scramble memory
 * 
 * All the functions listed below are the public accessible functions for the
 * Subscriber, Master, and Publisher classes. This is done because objects cannot
 * be returned to the Julia programming language and retain their connections to their
 * member functions. As a solution, we made the functions publiclly accessible and 
 * the first parameter of each function is the object the function is used for as
 * a mutable pointer variable. Any new publiclly accessible function for Publisher,
 * Subscriber, or Master must be placed here if it is desired to be availible in Julia.
 */

 /**
  * Replicates Connect in Master.rs
  * Returns a Master object connected to IP and port
  * IP (u32) - IP to connect to
  * port (u16) - port to connect to
  * Returns (Box<master::Master>) - Heap variable of Master
  */
#[no_mangle]
pub extern fn connectJ(IP: u32, port: u16) -> Box<master::Master>
 {
     let ip = Ipv4Addr::from(IP);
     let t = ip.to_owned();
     let m = master::connect(t.to_string(), port);
     return Box::new(m);
}

/**
 * Replicates setThreading in Master.rs
 * Enables or disables the multithreading of Master
 * target (*Master) - Master to set threading of
 * a (bool) - value to set threading to
 */
#[no_mangle]
pub extern fn setThread(target: &mut Master, a: bool)
{
    target.setThreading(a);
}

/**
 * Replicates subscriber in Master.rs
 * returns the user a new subscriber
 * m (*Master) - Master to get subscriber from
 * Returns (Box<master::subscriber::Subscriber>) - Subscriber object
 */
#[no_mangle]
pub extern fn subscriberJ(m: &mut Master) -> Box<master::subscriber::Subscriber>
 {
     let s = m.subscriber();
     return Box::new(s);
}

/**
 * Replicates publisher in Master.rs
 * returns the user a new publisher
 * m (*Master) - Master to get publisher from
 * Returns (Box<master::publisher::Publisher>) - publisher object
 */
#[no_mangle]
pub extern fn publisherJ(m: &mut Master) ->
Box<master::publisher::Publisher>
{
    let p = m.publisher();
    return Box::new(p);
}

/**
 * Replicates host in Master.rs
 * Hosts a new masterProcess
 * m (*Master) - Master to host from
 */
#[no_mangle]
pub extern fn hostJ(m: &mut Master)
{
    m.host()
}

/**
 * Replicates terminate in Master.rs
 * Terminates and stops a MasterProcess the Master is connected too
 * m (*Master) - Master to send terminate from
 */
#[no_mangle]
pub extern fn terminateJ(m: &mut Master)
{
    m.terminate()
}

/**
 * Replicates serialize in Master.rs
 * Returns a serialized string representing the current state
 * of the MasterProcess the Master is connected to
 * m (*Master) - Master to send serialize request from
 * Returns (*char) - C string of JSON
 */
#[no_mangle]
pub extern fn serializeJ(m: &mut Master) -> *const c_char
{
    let s = CString::new(m.serialize()).expect("Failure to get serialized Data").into_raw();
    s
}

/**
 * Removes a Cstring from the stack and places it on the heap
 * m (*char) - Stack Cstring
 */
#[no_mangle]
pub extern fn freeString(m: *mut c_char)
{
    let _c_string = unsafe{CString::from_raw(m)};
}

/**
 * Replicates setPortRanges in Master.rs
 * Sets the port ranges for new channels to a lower and upper bound
 * m (*Master) - Master to initiate request to MasterProcess from
 * lower (u16) - lower bound for port range
 * upper (u16) - upper bound for prot range
 */
#[no_mangle]
pub extern fn setPortRangesJ(m: &mut Master, lower: u16, upper: u16)
{
    m.setPortRanges(lower, upper);
}

/**
 * Replicates createChannel in Master.rs
 * Creates a new Channel
 * m (*Master) - Master to initiate request to MasterProcess from
 * port (u16) - Port to host new channel on
 * name (*char) - Name of channel
 * style (*char) - Style of channel
 * messageLimit (u32) - Limit of messages in Channel
 */
#[no_mangle]
pub extern fn createChannelJ(m: &mut Master, port: u16, name: *const c_char, style: *const c_char, messageLimit: u32)
{
    let n = unsafe { CStr::from_ptr(name) };
    let s = unsafe { CStr::from_ptr(style) };

    m.createChannel(port, n.to_str().unwrap().to_string(), s.to_str().unwrap().to_string(), messageLimit);
}

/**
 * Replicates getChannelTypes in Master.rs
 * Prints out supported Channel types
 * m (*Master) - Master to initiate request to MasterProcess from
 */
#[no_mangle]
pub extern fn getChannelTypesJ(m: &mut Master)
{
   m.getChannelTypes();
}

/**
 * Replicates removeChannel in Master.rs
 * Removes a channel of a given name from MasterProcess
 * m (*Master) - Master to initiate request to MasterProcess from
 * name (*char) - Name of Channel to remove
 */
#[no_mangle]
pub extern fn removeChannelJ(m: &mut Master, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    m.removeChannel(n.to_str().unwrap().to_string());
}

/**
 * Connects a Publisher to a Channel
 * p (*Publisher) - Publisher to connect from
 * name (*char) - Channel to connect to
 * */
#[no_mangle]
pub extern fn connectPJ(p: &mut Publisher, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    p.connect(n.to_str().unwrap().to_string());
}

/**
 * Connects a Subscriber to a Channel
 * s (*Subscriber) - Subscriber to connect from
 * name (*char) - Name of Channel to connect to
 */
#[no_mangle]
pub extern fn connectSJ(s: &mut Subscriber, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    s.connect(n.to_str().unwrap().to_string());
}

/**
 * Disconnects a Publisher from a Channel
 * p (*Publisher) - Publisher to disconnect
 * name (*char) - Channel to disconnect from
 * */
#[no_mangle]
pub extern fn disconnectPJ(p: &mut Publisher, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    p.disconnect(n.to_str().unwrap().to_string());
}

/**
 * Disconnects a Subscriber from a Channel
 * s (*Subscriber) - Subscriber to disconnect
 * name (*char) - Channel to disconnect from
 * */
#[no_mangle]
pub extern fn disconnectSJ(s: &mut Subscriber, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    s.disconnect(n.to_str().unwrap().to_string());
}

/**
 * Sends data from a Publsiher to a Channel
 * p (*Publisher) - Publisher to send from
 * chan (*char) - Name of channel to send to
 * msg (*char) - Message to send
 */
#[no_mangle]
pub extern fn publishJ(p: &mut Publisher, chan: *const c_char, msg: *const c_char)
{
    let c = unsafe { CStr::from_ptr(chan) };
    let m = unsafe { CStr::from_ptr(msg) };
    p.publish(c.to_str().unwrap().to_string(),m.to_str().unwrap().to_string());
}

/**
 * Recieves data from a Channel to a subscriber
 * s (*Subscriber) - Subscriber to initiate recieve request to channel
 * chan (*char) - Name of Channel to listen to
 * Returns (*char) - String of data in Channel
 */
#[no_mangle]
pub extern fn listenJ(s: &mut Subscriber, chan: *const c_char) -> *const c_char
{
    let c = unsafe {CStr::from_ptr(chan) };
    let m = CString::new(s.listen(c.to_str().unwrap().to_string())).expect("Failure to get serialized Data").into_raw();
    m
}
