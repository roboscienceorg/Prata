use super::channel;
pub fn test()
{
    basic_test();
    remove_ip_test();
    add_data_test();
    remove_data_test();
    add_ip_ports_test();
}
fn add_ip_ports_test()
{
    //println!("Test - Channel - Add IP and Ports");
    let mut c = channel::Channel::new("test".to_string(),55555);
    c.add("192.168.0.0".to_string());
    let ports = c.getPorts("192.168.0.0".to_string());

    assert!(ports.fullRange == true, "TEST: Added single port to list, full range was not set")
    //assert!(ports.fullRange == false, "TEST: Added single port to list, full range was blocked")
}
fn remove_data_test()
{
    //println!("Test - Channel - Remove Data");
    let mut c = channel::Channel::new("test".to_string(),55555);

    let data = c.getData();
    assert!(data == "", "TEST: Constructs with data already inside FAIL");

}
fn add_data_test()
{
    //println!("Test - Channel - Add Data");
    let mut c = channel::Channel::new("test".to_string(),55555);
    c.addData("data element 1".to_string());
    c.addData("data element 2".to_string());
    c.addData("data element 3".to_string());
    c.addData("data element 4".to_string());

    //println!("{:?}",c.getData());
    assert!(c.getData() == "data element 1data element 2", "TEST: get data fail");

    c.addData("the big\nhouse".to_string());
    c.addData("\"what\" are you doing".to_string());
    c.addData("\t\t\r\n".to_string());
    c.addData("2 9 {{}}".to_string());

    assert!(c.getData() == "data element 3data element 4", "TEST: get data 2 fail")
}
fn basic_test()
{
    //println!("Test - Channel - Basic");
    let mut c = channel::Channel::new("test".to_string(),55555);
    let listed = c.getListed();
    assert!(listed.len() == 0, "TEST: Default constructor starts with ip listed");

    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 1, "TEST: Adding does not increase size by 1");

    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 1, "TEST: Adding duplicate increases size by 1");

    c.add("192.168.0.0".to_string());
    c.add("192.168.0.1".to_string());
    c.add("192.168.0.2".to_string());
    c.add("192.168.0.3".to_string());
    c.add("192.168.0.0".to_string());
    c.add("192.168.0.0".to_string());
    let listed = c.getListed();
    assert!(listed.len() == 4, "TEST: Adding additional adresses then duplicates fails");

}
fn remove_ip_test()
{
    //println!("Test - Channel - Remove Address");
    let mut c = channel::Channel::new("test".to_string(),55555);
    c.add("192.168.0.1".to_string());
    c.add("192.168.0.2".to_string());
    c.add("192.168.0.3".to_string());
    c.add("192.168.0.0".to_string());
    c.add("192.168.0.0".to_string());
    c.remove("192.168.0.0".to_string());
    c.remove("192.168.0.2".to_string());

    assert!(c.count() == 2, "TEST: Removing fail");

    c.remove("192.168.0.1".to_string());
    c.remove("192.168.0.3".to_string());

    assert!(c.count() == 0, "TEST: Removing fail");

    c.remove("192.168.0.1".to_string());
    c.remove("192.168.0.3".to_string());

    assert!(c.count() == 0, "TEST: Removing from empty fail");

}
