use std::{collections::HashMap, fs::read_to_string, net::ToSocketAddrs, ptr::{null, NonNull}};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,            //地图ID
    pub name: String,       //地图名称
    pub look: String,       //地图展示    
    pub lookat: HashMap<String, String>, //地图内更多的命令
    pub looks: HashMap<u32, String>, //一个地点有多个展示 
    pub climbat: HashMap<String, String>, //爬山动作
    pub knockat: HashMap<String, String>, //敲门动作
    pub destpos: u32,   //目标位置
    pub localmaps: String,  //地图
    pub east_id: u32,       
    pub west_id: u32,
    pub south_id: u32,
    pub north_id: u32,
    pub northeast_id: u32,
    pub northwest_id: u32,
    pub southeast_id: u32,
    pub southwest_id: u32,
    pub easts: HashMap<u32, u32>,
    pub wests: HashMap<u32, u32>,
    pub souths: HashMap<u32, u32>,
    pub norths: HashMap<u32, u32>,
    pub northeasts: HashMap<u32, u32>,
    pub northwests: HashMap<u32, u32>,
    pub southeasts: HashMap<u32, u32>,
    pub southwests: HashMap<u32, u32>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: 0,
            name: String::from(""),
            look: String::from(""),
            lookat: HashMap::new(),
            looks: HashMap::new(),
            climbat: HashMap::new(),
            knockat: HashMap::new(),
            destpos: 0,
            localmaps: String::from(""),
            east_id: 0,
            west_id: 0,
            south_id: 0,
            north_id: 0,
            northeast_id: 0,
            northwest_id: 0,
            southeast_id: 0,
            southwest_id: 0,   
            easts: HashMap::new(),
            wests: HashMap::new(),
            souths: HashMap::new(),
            norths: HashMap::new(),
            northeasts: HashMap::new(),
            northwests: HashMap::new(),
            southeasts: HashMap::new(),
            southwests: HashMap::new(),         
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

trait Walk {
    fn east(&self);
}

impl Walk for Node {
    fn east(&self){}
}

pub fn init_map() -> HashMap<u32, Node> {

    let mut nodes: HashMap<u32, Node> = HashMap::new();

    let buf = match read_to_string("maps/liuxiu.txt"){
        Ok(a) => a,
        Err(_) => "".to_string(),
    };

    let n_group: Vec<&str> = buf.split("[node]").collect();
    
    for n in n_group.iter() {
        if !n.contains("=") {
            continue;
        }

        let mut node = Node::new();             
        for i in n.lines() {
            let group: Vec<&str> = i.split("=").collect();
            let key = match group.get(0) {
                Some(a) => a,
                None => "none",
            };
            
            let item = match group.get(1){
                Some(a) => a,
                None => "",
            };
            match key {
                "id" => {node.id = item.parse().unwrap(); },
                "name" => {node.name = item.to_string(); },
                "look" => {node.look = item.to_string(); },
                "look@river" | "look@path" => {
                    let cmds: Vec<&str> = key.split("@").collect();
                    let cmd = match cmds.get(1) {
                        Some(a) => a,
                        None => "",
                    };
                    node.lookat.insert(cmd.to_string(), item.to_string());
                },       
                "climb@up" | "climb@updone" | "climb@down" => {
                    let cmds: Vec<&str> = key.split("@").collect();
                    let cmd = match cmds.get(1) {
                        Some(a) => a,
                        None => "",
                    };
                    node.climbat.insert(cmd.to_string(), item.to_string());
                },
                "knock@gate" | "knock@gatedone" => {
                    let cmds: Vec<&str> = key.split("@").collect();
                    let cmd = match cmds.get(1) {
                        Some(a) => a,
                        None => "",
                    };
                    node.knockat.insert(cmd.to_string(), item.to_string());
                },
                "destpos" => {node.destpos = item.parse().unwrap(); }
                "localmaps" => {node.localmaps = item.to_string();}
                "east" => {node.east_id = item.parse().unwrap(); },
                "west" => {node.west_id = item.parse().unwrap(); },
                "south" => {node.south_id = item.parse().unwrap(); },
                "north" => {node.north_id = item.parse().unwrap(); },
                "southeast" => {node.southeast_id = item.parse().unwrap(); },
                "southwest" => {node.southwest_id = item.parse().unwrap(); },
                "northeast" => {node.northeast_id = item.parse().unwrap(); },
                "northwest" => {node.northwest_id = item.parse().unwrap(); },
                "east$1" => {node.easts.insert(1, item.parse().unwrap());},

                _ => (),
            };

            let mut cnt = 0;
            loop {
                if cnt > 3 {
                    break;
                }

                cnt += 1;
                let content = "$".to_string() + &cnt.to_string();
                if key.contains(&content) {
                    let keys:Vec<&str> = key.split("$").collect();
                    let key_cmd = match keys.get(0) {
                        Some(a) => a,
                        None => "none",
                    };

                    match key_cmd {
                        "look" => {
                            let cmds: Vec<&str> = key.split("$").collect();
                            let cmd = match cmds.get(1) {
                                Some(a) => a,
                                None => "",
                            };
                            node.looks.insert(cnt, item.to_string());
                        },
                        "east" => {node.easts.insert(cnt, item.parse().unwrap()); },
                        "west" => {node.wests.insert(cnt, item.parse().unwrap()); },
                        "south" => {node.souths.insert(cnt, item.parse().unwrap()); },
                        "north" => {node.norths.insert(cnt, item.parse().unwrap());},
                        "southeast" => {node.southeasts.insert(cnt, item.parse().unwrap()); },
                        "southwest" => {node.southwests.insert(cnt, item.parse().unwrap()); },
                        "northeast" => {node.northeasts.insert(cnt, item.parse().unwrap()); },
                        "northwest" => {node.northwests.insert(cnt, item.parse().unwrap()); },
                        _ => (),
                    };
                }
            }
        }

        nodes.insert(node.id, node);
    }

    // let node_display = nodes.clone();
    // for item in node_display.iter() {
    //     println!("{}", item.1.name);
    // }

    nodes
}