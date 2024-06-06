use std::{collections::HashMap, fs::read_to_string, net::ToSocketAddrs, ptr::{null, NonNull}};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub look: String,
    pub lookat: Vec<String>,
    pub east_id: u32,
    pub west_id: u32,
    pub south_id: u32,
    pub north_id: u32,
    pub northeast_id: u32,
    pub northwest_id: u32,
    pub southeast_id: u32,
    pub southwest_id: u32,
    pub east: Option<NonNull<Node>>,
    pub west: Option<NonNull<Node>>,
    pub south: Option<NonNull<Node>>,
    pub north: Option<NonNull<Node>>,
    pub northeast: Option<NonNull<Node>>,
    pub northwest: Option<NonNull<Node>>,
    pub southeast: Option<NonNull<Node>>,
    pub southwest: Option<NonNull<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: 0,
            name: String::from(""),
            look: String::from(""),
            lookat: Vec::new(),
            east_id: 0,
            west_id: 0,
            south_id: 0,
            north_id: 0,
            northeast_id: 0,
            northwest_id: 0,
            southeast_id: 0,
            southwest_id: 0,
            east: None,
            west: None,
            south: None,
            north: None,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
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
            
            match key {
                "id" => {node.id = group.get(1).unwrap().parse().unwrap(); },
                "name" => {node.name = group.get(1).unwrap().to_string(); },
                "look" => {node.look = group.get(1).unwrap().to_string(); },
                "east" => {node.east_id = group.get(1).unwrap().parse().unwrap(); },
                "west" => {node.west_id = group.get(1).unwrap().parse().unwrap(); },
                "south" => {node.south_id = group.get(1).unwrap().parse().unwrap(); },
                "north" => {node.north_id = group.get(1).unwrap().parse().unwrap(); },
                "southeast" => {node.southeast_id = group.get(1).unwrap().parse().unwrap(); },
                "southwest" => {node.southwest_id = group.get(1).unwrap().parse().unwrap(); },
                "northeast" => {node.northeast_id = group.get(1).unwrap().parse().unwrap(); },
                "northwest" => {node.northwest_id = group.get(1).unwrap().parse().unwrap(); },
                _ => (),
            };
        }

        nodes.insert(node.id, node);
    }

    let node_display = nodes.clone();
    for item in node_display.iter() {
        println!("{}", item.1.name);
    }

    nodes
}