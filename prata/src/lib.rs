#![allow(non_snake_case)]
#[allow(dead_code)]
extern crate pyo3;
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
//use std::net::Ipv4Addr;
//use port_scanner::request_open_port;

use master::Master;
use publisher::Publisher;
use subscriber::Subscriber;



#[pymodule]
fn prata(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    #[pyfn(m, "connect")]
    fn connect(py: Python, IP: String, p: u16) -> Py<Master>{

        // let p = request_open_port().unwrap_or(0);
        // let addr = (Ipv4Addr::LOCALHOST).to_string();
        Py::new(py, Master{ipAddress: IP, port: p, threading: true}).unwrap()
    }

    m.add_class::<Master>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Publisher>()?;

    Ok(())
}



use std::ffi::{CString, CStr};
use std::os::raw::c_char;



#[no_mangle]
pub extern fn connectJ(IP: u32, port: u16) -> Box<master::Master>
 {
     let ip = Ipv4Addr::from(IP);
     let t = ip.to_owned();
     let m = master::connect(t.to_string(), port);
     return Box::new(m);
}

#[no_mangle]
pub extern fn setThread(target: &mut Master, a: bool)
{
    target.setThreading(a);
}

#[no_mangle]
pub extern fn subscriberJ(m: &mut Master) -> Box<master::subscriber::Subscriber>
 {
     let s = m.subscriber();
     return Box::new(s);
}

#[no_mangle]
pub extern fn publisherJ(m: &mut Master) ->
Box<master::publisher::Publisher>
 {
     let p = m.publisher();
     return Box::new(p);
}


#[no_mangle]
pub extern fn hostJ(m: &mut Master)
 {
     m.host()
 }

#[no_mangle]
pub extern fn terminateJ(m: &mut Master)
 {
     m.terminate()
 }

 #[no_mangle]
 pub extern fn serializeJ(m: &mut Master) -> *const c_char
  {
      let s = CString::new(m.serialize()).expect("Failure to get serialized Data").into_raw();
      s
  }

  #[no_mangle]
  pub extern fn freeString(m: *mut c_char)
   {
       let _c_string = unsafe{CString::from_raw(m)};
   }

  #[no_mangle]
  pub extern fn setPortRangesJ(m: &mut Master, lower: u16, upper: u16)
   {
       m.setPortRanges(lower, upper);
   }

   #[no_mangle]
   pub extern fn createChannelJ(m: &mut Master, port: u16, name: *const c_char, style: *const c_char, messageLimit: u32)
    {
        let n = unsafe { CStr::from_ptr(name) };
        let s = unsafe { CStr::from_ptr(style) };

        m.createChannel(port, n.to_str().unwrap().to_string(), s.to_str().unwrap().to_string(), messageLimit);
    }


#[no_mangle]
pub extern fn getChannelTypesJ(m: &mut Master)
{
   m.getChannelTypes();
}

#[no_mangle]
pub extern fn removeChannelJ(m: &mut Master, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    m.removeChannel(n.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn connectPJ(p: &mut Publisher, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    p.connect(n.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn connectSJ(s: &mut Subscriber, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    s.connect(n.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn disconnectPJ(p: &mut Publisher, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    p.disconnect(n.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn disconnectSJ(s: &mut Subscriber, name: *const c_char)
{
    let n = unsafe { CStr::from_ptr(name) };
    s.disconnect(n.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn publishJ(p: &mut Publisher, chan: *const c_char, msg: *const c_char)
{
    let c = unsafe { CStr::from_ptr(chan) };
    let m = unsafe { CStr::from_ptr(msg) };
    p.publish(c.to_str().unwrap().to_string(),m.to_str().unwrap().to_string());
}

#[no_mangle]
pub extern fn listenJ(s: &mut Subscriber, chan: *const c_char) -> *const c_char
{
     let c = unsafe {CStr::from_ptr(chan) };
    let m = CString::new(s.listen(c.to_str().unwrap().to_string())).expect("Failure to get serialized Data").into_raw();
    m
}
