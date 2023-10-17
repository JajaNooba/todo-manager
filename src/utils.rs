#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub is_complete: bool,
    pub is_important: bool
}

impl Task {
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.name);
        str.push('\n');
        str.push_str(&self.description);
        str.push('\n');
        if self.is_complete { 
            str.push_str("true\n") 
        } else { 
            str.push_str("false\n") 
        };
        if self.is_important { 
            str.push_str("true\n") 
        } else { 
            str.push_str("false\n")
        };
        str
    }
}