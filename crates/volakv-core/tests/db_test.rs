use volakv_core::Db;
use bytes::Bytes;

#[test]
fn test_set_and_get() {
    let db = Db::new();
    
    let key = "key1";
    let value = Bytes::from("value1");

    db.set(key.to_string(), value.clone());
    
    let fetched = db.get(key);
    assert_eq!(fetched, Some(value));
}

#[test]
fn test_get_non_existent_key() {
    let db = Db::new();
    assert_eq!(db.get("missing_key"), None);
}

#[test]
fn test_overwrite_value() {
    let db = Db::new();
    let key = "key_overwrite";

    db.set(key.to_string(), Bytes::from("old_val"));
    assert_eq!(db.get(key), Some(Bytes::from("old_val")));

    db.set(key.to_string(), Bytes::from("new_val"));
    assert_eq!(db.get(key), Some(Bytes::from("new_val")));
}

#[test]
fn test_delete_key() {
    let db = Db::new();
    let key = "key_to_delete";
    let value = Bytes::from("temp_val");

    db.set(key.to_string(), value.clone());
    assert!(db.exists(key));

    // del should return true if key existed and was deleted
    let deleted = db.del(key);
    assert!(deleted);
    assert_eq!(db.get(key), None);
    assert!(!db.exists(key));

    // del on non-existent key should return false
    let deleted_again = db.del(key);
    assert!(!deleted_again);
}

#[test]
fn test_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let db = Arc::new(Db::new());
    let mut handles = vec![];

    for i in 0..10 {
        let db_clone = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let key = format!("thread_key_{}", i);
            let val = Bytes::from(format!("thread_val_{}", i));
            db_clone.set(key.clone(), val.clone());
            assert_eq!(db_clone.get(&key), Some(val));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    assert_eq!(db.len(), 10);
}
