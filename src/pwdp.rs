use persist::*;

pub fn search<'a>(search_for:&str, store:&'a Store) -> Vec<&'a Entry> {
    return vec![];
}

#[test]
fn test_store_search_title() {
    let hit = Entry { 
            title: "abc hit abc".to_string(), 
            username: "user".to_string(),
            password: "password".to_string(),
            link: "link".to_string(),
            additional: "additional".to_string() 
        };  

    let miss =  Entry { 
            title: "abc something".to_string(), 
            username: "user".to_string(),
            password: "password".to_string(),
            link: "link".to_string(),
            additional: "additional".to_string() 
        }; 

    let mut store = Store::new();
    store.add(hit);
    store.add(miss);
    let hits = search("hit", &store);
    assert_eq!(hits.len(), 1);
    assert_eq!(*hits[0], hit); 
}