#[derive(PartialEq, Clone, Debug)]
pub struct Entry {
    title: String,
    username: String,
    password: String,
    link: String,
    additional: String
}

pub struct Store {
    entries : Vec<Entry>
}

impl Store {
    pub fn new() -> Store {
        Store { entries: vec![] }
    }

    pub fn add(&mut self, entry:Entry) {
        self.entries.push(entry);
    }
}

#[test]
fn test_store_starts_empty() {
    let s = Store::new();
    assert_eq!(s.entries.len(), 0);
}

#[test]
fn test_add_entry_adds_entry() {
    let mut s = Store::new();
    let entry = Entry { 
            title: "hello".to_string(), 
            username: "user".to_string(),
            password: "password".to_string(),
            link: "link".to_string(),
            additional: "additional".to_string() 
        };
    s.add(entry.clone());
    assert_eq!(s.entries.len(), 1);
    assert_eq!(s.entries[0], entry);
}