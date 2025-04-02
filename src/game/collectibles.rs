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
            "ruby" => Self::Ruby,
            "diamond" => Self::Diamond,
            "red" => Self::Red,
            "loli" => Self::Loli,
            "cup" => Self::Cup,
            "yussuk" => Self::Yussuk,
            "king" => Self::King,
            _ => panic!("Invalid collectible type: {name}"),
        }
    }

    pub const fn data(&self) -> CollectibleData {
        match self {
            Self::Ruby => CollectibleData {
                offset: 0.0,
                value: 50,
            },
            Self::Diamond => CollectibleData {
                offset: 32.0,
                value: 100,
            },
            Self::Red => CollectibleData {
                offset: 64.0,
                value: 150,
            },
            Self::Loli => CollectibleData {
                offset: 96.0,
                value: 400,
            },
            Self::Cup => CollectibleData {
                offset: 128.0,
                value: 1000,
            },
            Self::Yussuk => CollectibleData {
                offset: 160.0,
                value: 600,
            },
            Self::King => CollectibleData {
                offset: 192.0,
                value: 700,
            },
        }
    }
}
