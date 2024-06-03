use crate::player::Player;
use crate::service::Login;
use crate::service::LoginInfo;
use crossbeam::channel::Sender;
use crate::channel::Message;
use crate::channel::wrap_message;
use std::io::Error;

pub fn do_login(
    s_service: &Sender<String>, 
    login_info: &mut LoginInfo,
    player: &mut Player,
    msg: &Message
) -> Result<u32, Error>
{
    //todo: 重复用户登录判断

    //登录名还未赋值
    if login_info.login.login_name.is_empty() {

        //从文件中读取用户
        let user_file = utils::load_file("login.json");
        let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
        let filter: Vec<&Login> = logins.iter()
            .filter(|item| item.login_name == msg.content)
            .collect();

        //用户存在
        if filter.len() > 0 {
            login_info.login.login_name = msg.content.to_string();
            let val = wrap_message(msg.addr, "此ID档案已存在,请输入密码:".to_string());
            s_service.send(val).unwrap(); 
            return Ok(1);
        } else { //用户不存在
            let val = wrap_message(msg.addr, "用户不存在".to_string());
            s_service.send(val).unwrap();
            return Ok(2);
        }
    } else {
        //直接就是密码
        login_info.login.password = msg.content.to_string();

        let user_file = utils::load_file("login.json");
        let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
        let filter: Vec<&Login> = logins.iter()
            .filter(|item| 
                item.login_name == login_info.login.login_name &&
                item.password == login_info.login.password
            )
            .collect();
        
        // println!("{} {}", login.login_name, login.password);
        if filter.len() > 0 {
            login_info.b_login = true;
            let val = wrap_message(msg.addr, "重新连线完毕。".to_string());
            s_service.send(val).unwrap(); 

            //获取用户资料
            let user_file = utils::load_file("users.json");
            let users: Vec<Player> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
            let filter: Vec<&Player> = users.iter()
                .filter(|item| item.name == login_info.login.login_name)
                .collect();

            let filter_player = filter[0].clone();
            *player = filter_player;

        } else { //用户不存在
            let val = wrap_message(msg.addr, "密码错误！".to_string());
            s_service.send(val).unwrap();
            return Ok(4);
        }
    }

    Ok(0)
}
