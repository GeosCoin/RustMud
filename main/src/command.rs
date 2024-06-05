use std::collections::HashMap;

pub trait Command{
    fn execute(&self) -> String;
}

struct EmptyCommand{}

impl EmptyCommand {
    fn new() -> Self {
        EmptyCommand{}
    }
}

impl Command for EmptyCommand {
    fn execute(&self) -> String {
        "".to_string()
    }
}

pub(crate) struct Invoker<'a> {
    command: Box<dyn Command + 'a>
}

impl<'a> Invoker<'a> {
    pub fn new() -> Self{  
        Invoker {
            command: Box::new(EmptyCommand::new())
        }
    }

    pub fn set(&mut self, command: Box<dyn Command + 'a>){  
        self.command = command;
    }

    pub fn execute(&self) -> String {
        self.command.execute()
    }
}

// Command pattern example: 
// pub(crate) struct Invokers<'a> {
//     commands: HashMap<String, Box<dyn Command + 'a>>,
//     cur_command: String,
// }

// impl<'a> Invokers<'a> {
//     pub fn new(command: &'a String) -> Self{   
//         let cmd = command.split(" ").collect::<Vec<&str>>();
//         let cmd = match cmd.get(0) {
//             Some(a) => a,
//             None => "none",
//         };
//         Invokers  {
//             commands: HashMap::new(),
//             cur_command: cmd.to_string()
//         }
//     }

//     pub fn add(&mut self, cmd_key:String, command: Box<dyn Command + 'a>) {
//         self.commands.insert(cmd_key, command);
//     }

//     pub fn execute(&self) -> String {
//         match self.commands.get(&self.cur_command) {
//             Some(a) => a.execute(),
//             None => {return "none".to_string();}
//         }
//     }
// }

