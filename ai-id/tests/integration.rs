extern crate serde_json;
extern crate ai-id;
extern crate hex;

static FIXTURES: &'static str = include_str!("../../test/fixtures.json");

fn test_correct(e: &ai-id::aiidEncoding, id: &str, data: &[u8]) {
    assert!(!e.is_corrupt(id).unwrap());
    let r = e.decode(id)
        .unwrap_or_else(|err| panic!(format!(
            "correct test of {} failed: {:?}", id, err)));
    assert_eq!(data, r.as_slice());
    let r = e.encode(data).unwrap();
    assert_eq!(id, r);
}

fn test_correctable(e: &ai-id::aiidEncoding, id: &str, data: &[u8], correct_id: &str) {
    assert!(e.is_corrupt(id).unwrap());
    let r = e.decode(id)
        .unwrap_or_else(|err| panic!(format!(
            "correctable test of {} failed: {:?}", id, err)));
    assert_eq!(data, r.as_slice());
    let r = e.encode(&r).unwrap();
    assert_eq!(correct_id, r);
}

fn test_errant_id(e: &ai-id::aiidEncoding, id: &str, err: &str) {
    assert!(e.is_corrupt(id).unwrap());
    let r = e.decode(id).unwrap_err();
    assert_eq!(err, format!("{:?}", r));
}

fn test_errant_data(e: &ai-id::aiidEncoding, data: &[u8], err: &str) {
    let r = e.encode(data).unwrap_err();
    assert_eq!(err, format!("{:?}", r));
}

fn test(e: &ai-id::aiidEncoding, test: &serde_json::Value) {
    let test = test.as_object().unwrap();

    for t in test["correct"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let data = hex::decode(&String::from(t[1].as_str().unwrap())).unwrap();
        test_correct(e, &id, &data);
    }

    for t in test["correctable"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let data = hex::decode(&String::from(t[1].as_str().unwrap())).unwrap();
        let correct_id = String::from(t[2].as_str().unwrap());
        test_correctable(e, &id, &data, &correct_id);
    }

    for t in test["errantId"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let err = String::from(t[1].as_str().unwrap());
        test_errant_id(e, &id, &err);
    }

    for t in test["errantData"].as_array().unwrap().iter() {
        let data = hex::decode(&String::from(t[0].as_str().unwrap())).unwrap();
        let err = String::from(t[1].as_str().unwrap());
        test_errant_data(e, &data, &err);
    }
}

#[test]
fn it_can_execute_fixtures() {
    let fixtures: serde_json::Value = serde_json::from_str(FIXTURES).unwrap();
    let fixtures = fixtures.as_object().unwrap();

    let e = ai-id::aiidEncoding::with_kind("aik0").unwrap();
    test(&e, &fixtures["aik0"]);

    let e = ai-id::aiidEncoding::with_kind("aia0").unwrap();
    test(&e, &fixtures["aia0"]);

    let e = ai-id::aiidEncoding::with_kind("ais0").unwrap();
    test(&e, &fixtures["ais0"]);
}
