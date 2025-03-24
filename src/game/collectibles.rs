pub struct CollectibleData {
    pub offset: f32,
    pub value: u32,
}

pub enum CollectibleType {
    Ruby,
    Diamond,
    Red,
    Loli,
    Cup,
    Yussuk,
    King,
}

impl CollectibleType {
    pub fn from(name: &str) -> Self {
        match name {
            "ruby" => CollectibleType::Ruby, 
            "diamond" => CollectibleType::Diamond,
            "red" => CollectibleType::Red,
            "loli" => CollectibleType::Loli,
            "cup" => CollectibleType::Cup,
            "yussuk" => CollectibleType::Yussuk,
            "king" => CollectibleType::King,
            _ => panic!("Invalid collectible type: {}", name),
        }
    }

    pub fn data(&self) -> CollectibleData {
        match self {
            CollectibleType::Ruby => CollectibleData { offset: 0.0, value: 50 },
            CollectibleType::Diamond => CollectibleData { offset: 32.0, value: 100 },
            CollectibleType::Red => CollectibleData { offset: 64.0, value: 150 },
            CollectibleType::Loli => CollectibleData { offset: 96.0, value: 400 },
            CollectibleType::Cup => CollectibleData { offset: 128.0, value: 1000 },
            CollectibleType::Yussuk => CollectibleData { offset: 160.0, value: 600 },
            CollectibleType::King => CollectibleData { offset: 192.0, value: 700 },
        }
    }
}
