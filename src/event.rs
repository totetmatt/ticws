use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    // Common Livecode Message Interface
    pub s:String,
    pub id:String,
    // END Common Livecode Message Interface

    pub data:String
}

