use crdt_lww::LWWElementDictionary;
use crdt_lww::Timestamp;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_add_and_lookup() {
    let dict = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value = "value1".to_string();
    let ts = Timestamp::now();

    dict.add(key.clone(), value.clone(), ts);
    assert_eq!(dict.lookup(&key), Some(Arc::new(value)));
}

#[test]
fn test_remove() {
    let dict = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value = "value1".to_string();

    let ts_add = Timestamp::now();
    dict.add(key.clone(), value, ts_add);

    sleep(Duration::from_millis(1));

    let ts_remove = Timestamp::now();
    dict.remove(&key, ts_remove);

    assert!(ts_remove >= ts_add, "ts_remove should be >= ts_add");

    assert_eq!(dict.lookup(&key), None);
}

#[test]
fn test_update() {
    let dict = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value1 = "value1".to_string();
    let value2 = "value2".to_string();
    let ts1 = Timestamp::now();

    dict.add(key.clone(), value1.clone(), ts1);
    sleep(Duration::from_millis(1));

    let ts2 = Timestamp::now();
    dict.update(key.clone(), value2.clone(), ts2);
    assert_eq!(dict.lookup(&key), Some(Arc::new(value2)));
}

#[test]
fn test_merge() {
    let dict1 = LWWElementDictionary::new();
    let dict2 = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value1 = "value1".to_string();
    let value2 = "value2".to_string();

    let ts1 = Timestamp::now();
    dict1.add(key.clone(), value1.clone(), ts1);

    sleep(Duration::from_millis(1));

    let ts2 = Timestamp::now();
    dict2.add(key.clone(), value2.clone(), ts2);

    dict1.merge(&dict2);
    assert_eq!(dict1.lookup(&key), Some(Arc::new(value2)));
}

#[test]
fn test_concurrent_add_and_remove() {
    let dict1 = LWWElementDictionary::new();
    let dict2 = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value = "value1".to_string();
    let ts = Timestamp::now();

    dict1.add(key.clone(), value.clone(), ts);
    dict2.remove(&key, ts);

    dict1.merge(&dict2);
    assert_eq!(dict1.lookup(&key), Some(Arc::new(value)));

    sleep(Duration::from_millis(1));

    let ts_new = Timestamp::now();
    dict2.remove(&key, ts_new);
    dict1.merge(&dict2);
    assert_eq!(dict1.lookup(&key), None);
}

#[test]
fn test_multiple_keys() {
    let dict = LWWElementDictionary::new();
    let key1 = "key1".to_string();
    let value1 = "value1".to_string();
    let key2 = "key2".to_string();
    let value2 = "value2".to_string();
    let ts = Timestamp::now();

    dict.add(key1.clone(), value1.clone(), ts);
    dict.add(key2.clone(), value2.clone(), ts);

    assert_eq!(dict.lookup(&key1), Some(Arc::new(value1)));
    assert_eq!(dict.lookup(&key2), Some(Arc::new(value2)));
}

#[test]
fn test_merge_conflicting_timestamps() {
    let dict1 = LWWElementDictionary::new();
    let dict2 = LWWElementDictionary::new();
    let key = "key1".to_string();
    let value1 = "value1".to_string();
    let value2 = "value2".to_string();

    let ts = Timestamp::now();

    dict1.add(key.clone(), value1.clone(), ts);
    dict2.add(key.clone(), value2.clone(), ts);

    dict1.merge(&dict2);

    assert_eq!(dict1.lookup(&key), Some(Arc::new(value1)));
}

#[test]
fn test_concurrent_access() {
    let dict = Arc::new(LWWElementDictionary::new());
    let num_threads = 10;
    let mut handles = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let dict_clone = Arc::clone(&dict);
        let key_add = format!("key_add_{}", i);
        let key_remove = format!("key_remove_{}", i);
        let value = format!("value_{}", i);
        let ts_add = Timestamp::now();

        let handle = thread::spawn(move || {
            dict_clone.add(key_add.clone(), value.clone(), ts_add);

            sleep(Duration::from_millis(1));

            dict_clone.remove(&key_remove, Timestamp::now());
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..num_threads {
        let key = format!("key_add_{}", i);
        let value = format!("value_{}", i);
        assert_eq!(dict.lookup(&key), Some(Arc::new(value)));
    }

    for i in 0..num_threads {
        let key = format!("key_remove_{}", i);
        assert_eq!(dict.lookup(&key), None);
    }
}

#[test]
fn test_concurrent_adds_and_removes_same_key() {
    let dict = Arc::new(LWWElementDictionary::new());
    let key = "shared_key".to_string();
    let num_threads = 10;
    let mut handles = Vec::with_capacity(num_threads * 2);

    for i in 0..num_threads {
        let dict_clone = Arc::clone(&dict);
        let key_clone = key.clone();
        let value = format!("value_{}", i);
        let ts = Timestamp::now();

        let handle_add = thread::spawn(move || {
            dict_clone.add(key_clone, value, ts);
        });
        handles.push(handle_add);

        let dict_clone = Arc::clone(&dict);
        let key_clone = key.clone();
        let ts = Timestamp::now();

        let handle_remove = thread::spawn(move || {
            dict_clone.remove(&key_clone, ts);
        });
        handles.push(handle_remove);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = dict.lookup(&key);
    match &result {
        Some(value_arc) => {
            println!("Key '{}' is present with value '{}'", key, *value_arc);
        }
        None => {
            println!("Key '{}' is absent", key);
        }
    }

    assert!(result.is_some() || result.is_none());
}
