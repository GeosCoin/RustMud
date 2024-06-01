use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

//业务处理入口
pub fn on_service(message: &str, s_service: Sender<String>){

    println!("in service, I got {}", message);

    if message.trim() == "lxz" {
        s_service.send("此ID档案已存在,请输入密码:".to_string()).unwrap();
        // Self::send(session,  "此ID档案已存在,请输入密码:");
        return;
    }

    // if message.trim() == "abc123" {
    //     Self::send(session,  "重新连线完毕。");
    //     return;
    // }

    // //新用户登录
    // let mut sessions_ok = sessions.lock().unwrap();
    
    // if message.trim() == "upgrade" {
    //     sessions_ok.entry(addr)
    //         .and_modify(|ctx| {
    //             ctx.player.name = "龙年".to_string();
    //             ctx.player.level = 30;
    //         });
    // } 

    // let player = &sessions_ok.get(&addr).unwrap().player;
    
    // let hpframe = r"
    // ┌─── ".to_owned() + &utils::show_color(&player.name, Color::YELLOW) + "状态────────────┬───────────────────┐
    // │【精神】 "+ &utils::show_color(&player.level.to_string(), Color::RED) +"     / 125      [100%]    │【精力】 100     / 100     (+   0)    │
    // │【气血】 17      / 127      [100%]    │【内力】 141     / 71      (+   0)    │
    // │【真气】 0       / 0        [  0%]    │【战意】 100%               [正常]    │
    // │【食物】 0       / 300      [饥饿]    │【潜能】 5075                         │
    // │【饮水】 0       / 300      [饥渴]    │【经验】 830                          │
    // ├───────────────────┴───────────────────┤
    // │【状态】 健康                                                                 │
    // └──────────────────────────────北大侠客行────┘\n>";

    // if message.trim() == "hp" {
    //     Self::send(session,  &hpframe);
    //     return;
    // }

    // if message.trim() == "l" {
        
    //     let mut other = 
    //         sessions_ok
    //         .iter()
    //         .filter(|p| !(p.0.to_string() == addr.to_string())) ;

    //     println!("{:?}", other.next().unwrap().1.player.name);
    // }

}