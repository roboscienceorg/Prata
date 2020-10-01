#![allow(non_snake_case)]
#[allow(dead_code)]
extern crate pyo3;
extern crate zmq;
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

mod channel;
mod master;
//mod publisher;
//mod subscriber;

use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;

// use std::str;
// use std::ffi::CStr;
// use std::os::raw::c_char;

//use splay::SplayMap;
//use splay::SplaySet;
//use std::thread;
//use std::time::Duration;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value as JsonValue;
//use std::collections::HashMap;


use master::Master;


#[pymodule]
fn TALA(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // m.add_function(wrap_pyfunction!(connect, m)?)?;
    // m.add_function(wrap_pyfunction!(disconnect, m)?)?;


    #[pyfn(m, "host")]
    fn host(_py: Python, ms: Master) -> PyResult<Master>{
        ms.host();
        Ok(ms)
    }

    #[pyfn(m, "connect")]
    fn connect(py: Python, IP: String, p: u16) -> Py<Master>{
        Py::new(py, Master{ipAddress: IP, port: p}).unwrap()
    }

    #[pyfn(m, "disconnect")]
    fn disconnect(_py: Python, ms: Master) -> PyResult<Master>{
        ms.disconnect();
        Ok(ms)
    }

    /*
    #[pyfn(m, "subscriber")]
    fn subscriber(py: Python, chn: String, ms: Master) -> Py<Subscriber>{
        Py::new(py, ms.subscriber(chn)).unwrap()
    }
    */
    /*
    #[pyfn(m, "publisher")]
    fn publisher(py: Python, chn: String, ms: Master) -> Py<Publisher>{
        Py::new(py, ms.publisher(chn)).unwrap()
    }

    #[pyfn(m, "pubLeave")]
    fn pubLeave(_py: Python, pb: Publisher, chan: String) -> PyResult<Publisher>{
        pb.disconnect(chan);
        Ok(pb)
    }

    #[pyfn(m, "subLeave")]
    fn subLeave(_py: Python, sb: Subscriber, chan: String) -> PyResult<Subscriber>{
        sb.disconnect(chan);
        Ok(sb)
    }


    #[pyfn(m, "pubJoin")]
    fn pubJoin(_py: Python, pb: Publisher, chan: String) -> PyResult<Publisher>{
        pb.connect(chan);
        Ok(pb)
    }

    #[pyfn(m, "subJoin")]
    fn subJoin(_py: Python, sb: Subscriber, chan: String) -> PyResult<Subscriber>{
        sb.connect(chan);
        Ok(sb)
    }

*/
    m.add_class::<master::Master>()?;
   // m.add_class::<Subscriber>()?;
   // m.add_class::<Publisher>()?;

    Ok(())
}
