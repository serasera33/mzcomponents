use moonlight::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Favorites {
    pub name: String,
    pub colors: Vec<String>,
    pub dishes: Vec<String>
}

impl Favorites {
    pub fn init() -> Favorites {
        Favorites {
            name: String::from(""),
            colors: Vec::<String>::new(),
            dishes: Vec::<String>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    Log(Message),

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    Log(Message),

}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(msg: &str) -> Message {
        Message {
            text: String::from(msg)
        }
    }
}

