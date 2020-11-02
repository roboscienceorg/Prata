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