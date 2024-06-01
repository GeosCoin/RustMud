
pub enum PowerType {
    Warrior,
    Ranger,
    Magician
}

pub struct Powers {

}

pub struct Inventory {
    goods: Vec<u32>
}

pub struct Equipment {
    head: u32,
    hands: u32,
    torso: u32,
    artifact: u32,
    ring_left: u32,
    ring_right: u32,
    main_hand: u32,
    off_hand: u32,
    feet: u32,
}

pub struct Player {
    pub name: String,   
    pub level: u32,  
    physical: u32,
    mental: u32,
    offense: u32,
    defence: u32,
    hp: u32,
    mp: u32,    
    xp: u32,    
    max_hp: u32,
    max_mp: u32,
    max_xp: u32,    
    hp_regen: u32,    
    mp_regen: u32,
    accuracy: u32,
    avoidance: u32,
}

impl Player {
    pub fn new() -> Self {
        Player{
            name: String::from("成王败寇"),   
            level: 1,  
            physical: 0,
            mental: 0,
            offense: 0,
            defence: 0,
            hp: 0,
            mp: 0,    
            xp: 0,    
            max_hp: 0,
            max_mp: 0,
            max_xp: 0,    
            hp_regen: 0,    
            mp_regen: 0,
            accuracy: 0,
            avoidance: 0,
        }
    }
}
