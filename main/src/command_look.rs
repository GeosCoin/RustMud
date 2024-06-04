use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, Message}, command::Command, player::Player};

pub struct LookCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message
}

impl<'a> LookCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message
        ) -> LookCommand<'a>  {
            LookCommand {
            players,
            s_service,
            msg
        }
    }
}
impl<'a>  Command for LookCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let mut l_view = r"
        未明谷 -  
                    树林----未明谷----乱石阵    
                              ｜     
                           青石桥头             
    山谷中绿树成荫，却不见有多么明媚的花开于此，但你仍能闻见了远远飘来的花香。耳边听到了溪
水叮咚的声音，原来不远处有一条蜿蜒的小溪(river)，岸边似乎散落了一些物什。在山谷的北侧有条陡
峭的山坡(path)隐隐可以通向外界。
    你可以看看(look):river,path，。
    「初春」: 太阳无奈地缓缓挂向西边的才露新芽的树梢。

    这里明显的方向有 south、east 和 west。

    二个葫芦(Hu lu)
    二枚野果(Ye guo)
    普通百姓 博迪鸟(Birddy) 
    普通百姓 翻炒西瓜拌面(Esther) 
".to_string();

        let others: Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name != player.name)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;
        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();
        "ok".to_string()
    }
}