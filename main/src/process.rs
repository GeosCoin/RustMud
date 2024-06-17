use crossbeam::channel::Sender;

use crate::channel::{wrap_message, Message};

pub trait PostProcess {
    fn execute(&mut self) -> String;
}

pub struct ProcessNone<'a> {
    ret_str: String,
    s_service: &'a Sender<String>,
    msg: &'a Message,
}

impl<'a> ProcessNone<'a> {
    pub fn new(ret_str: String,
        s_service: &'a Sender<String>,
        msg: &'a Message,) -> Self {
        ProcessNone {
            ret_str,
            s_service,
            msg
        }
    }
}

impl<'a> PostProcess for ProcessNone<'a> {
    fn execute(&mut self) -> String {
        if self.ret_str == "none" {
            let nomatch = "There is no match command.";
            let val = wrap_message(self.msg.addr, nomatch.to_string());
            self.s_service.send(val).unwrap();
        }
        "".to_string()
    }
}