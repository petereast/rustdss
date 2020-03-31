// This will provide list support - eventually support blocking commands?
// Commands: LPUSH, RPUSH, LPOP, RPOP, RPOPLPUSH

use crate::CoreState;
use rustdss_data::{Key, RespData};

pub fn lpush(state: &mut CoreState, key: &Key, data: RespData) -> RespData {
    match state.keyval.get_mut(key) {
        Some(RespData::List(list)) => {
            list.push_front(data);
            RespData::Number(list.len() as i64)
        }
        Some(_) => RespData::wrong_type(),
        None => {
            // Create a new list with the one thing in it
            state
                .keyval
                .insert(key.clone(), RespData::List(vec![data].into()));
            RespData::Number(1)
        }
    }
}

pub fn lpop(state: &mut CoreState, key: &Key) -> RespData {
    match state.keyval.get_mut(key) {
        Some(RespData::List(list)) => list.pop_front().unwrap_or(RespData::nil()),
        Some(_) => RespData::wrong_type(),
        _ => RespData::nil(),
    }
}

pub fn rpush(state: &mut CoreState, key: &Key, data: RespData) -> RespData {
    match state.keyval.get_mut(key) {
        Some(RespData::List(list)) => {
            list.push_back(data);
            RespData::Number(list.len() as i64)
        }
        Some(_) => RespData::wrong_type(),
        None => {
            // Create a new list with the one thing in it
            state
                .keyval
                .insert(key.clone(), RespData::List(vec![data].into()));
            RespData::Number(1)
        }
    }
}

pub fn rpop(state: &mut CoreState, key: &Key) -> RespData {
    match state.keyval.get_mut(key) {
        Some(RespData::List(list)) => list.pop_back().unwrap_or(RespData::nil()),
        Some(_) => RespData::wrong_type(),
        _ => RespData::nil(),
    }
}

pub fn llen(state: &CoreState, key: &Key) -> RespData {
    state
        .keyval
        .get(key)
        .and_then(|entry| match entry {
            RespData::List(l) => Some(RespData::Number(l.len() as i64)),
            _ => None,
        })
        .unwrap_or(RespData::nil())
}
#[cfg(test)]
mod lpush_should {
    use super::*;
    use crate::CoreState;
    use std::collections::HashMap;

    #[test]
    fn create_a_new_list() {
        let key: String = "key".into();
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = lpush(&mut state, &key, RespData::SimpleStr("value".into()));

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![RespData::SimpleStr("value".into())].into()
            ))
        );
        assert_eq!(response, RespData::Number(1));
    }

    #[test]
    fn push_items_to_the_beginning_of_the_list() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(
            key.clone(),
            RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                ]
                .into(),
            ),
        );
        let mut state = CoreState { keyval };

        let response = lpush(
            &mut state,
            &key,
            RespData::SimpleStr("should_be_first".into()),
        );

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![
                    RespData::SimpleStr("should_be_first".into()),
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                ]
                .into()
            ))
        );
        assert_eq!(response, RespData::Number(3));
    }

    #[test]
    fn error_when_the_list_is_a_different_type() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::SimpleStr("not_a_list".into()));
        let mut state = CoreState { keyval };

        let response = lpush(
            &mut state,
            &key,
            RespData::SimpleStr("some_new_data".into()),
        );

        // Assert that the original item was not mutated
        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::SimpleStr("not_a_list".into()))
        );
        assert_eq!(response, RespData::wrong_type());
    }
}

#[cfg(test)]
mod rpush_should {
    use super::*;
    use crate::CoreState;
    use std::collections::HashMap;

    #[test]
    fn create_a_new_list() {
        let key: String = "key".into();
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = rpush(&mut state, &key, RespData::SimpleStr("value".into()));

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![RespData::SimpleStr("value".into())].into()
            ))
        );
        assert_eq!(response, RespData::Number(1));
    }

    #[test]
    fn push_items_to_the_end_of_the_list() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(
            key.clone(),
            RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                ]
                .into(),
            ),
        );
        let mut state = CoreState { keyval };

        let response = rpush(
            &mut state,
            &key,
            RespData::SimpleStr("should_be_last".into()),
        );

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                    RespData::SimpleStr("should_be_last".into()),
                ]
                .into()
            ))
        );
        assert_eq!(response, RespData::Number(3));
    }

    #[test]
    fn error_when_the_list_is_a_different_type() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::SimpleStr("not_a_list".into()));
        let mut state = CoreState { keyval };

        let response = rpush(
            &mut state,
            &key,
            RespData::SimpleStr("some_new_data".into()),
        );

        // Assert that the original item was not mutated
        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::SimpleStr("not_a_list".into()))
        );
        assert_eq!(response, RespData::wrong_type());
    }
}

#[cfg(test)]
mod rpop_should {
    use super::*;
    use crate::CoreState;
    use std::collections::HashMap;

    #[test]
    fn return_nil_when_the_list_doesnt_exist() {
        let key: String = "key".into();
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = rpop(&mut state, &key);

        assert_eq!(state.keyval.get(&key), None);
        assert_eq!(response, RespData::nil());
    }

    #[test]
    fn pop_items_from_the_end_of_the_list() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(
            key.clone(),
            RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                    RespData::SimpleStr("should_be_last".into()),
                ]
                .into(),
            ),
        );
        let mut state = CoreState { keyval };

        let response = rpop(&mut state, &key);

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                ]
                .into()
            ))
        );
        assert_eq!(response, RespData::SimpleStr("should_be_last".into()));
    }

    #[test]
    fn returns_nil_when_the_list_is_empty() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::List(vec![].into()));
        let mut state = CoreState { keyval };

        let response = rpop(&mut state, &key);

        assert_eq!(state.keyval.get(&key), Some(&RespData::List(vec![].into())));
        assert_eq!(response, RespData::nil());
    }

    #[test]
    fn error_when_the_list_is_a_different_type() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::SimpleStr("not_a_list".into()));
        let mut state = CoreState { keyval };

        let response = rpop(&mut state, &key);

        // Assert that the original item was not mutated
        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::SimpleStr("not_a_list".into()))
        );
        assert_eq!(response, RespData::wrong_type());
    }
}

#[cfg(test)]
mod lpop_should {
    use super::*;
    use crate::CoreState;
    use std::collections::HashMap;

    #[test]
    fn return_nil_when_the_list_doesnt_exist() {
        let key: String = "key".into();
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = lpop(&mut state, &key);

        assert_eq!(state.keyval.get(&key), None);
        assert_eq!(response, RespData::nil());
    }

    #[test]
    fn pop_items_from_the_end_of_the_list() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(
            key.clone(),
            RespData::List(
                vec![
                    RespData::SimpleStr("first".into()),
                    RespData::SimpleStr("second".into()),
                    RespData::SimpleStr("should_be_last".into()),
                ]
                .into(),
            ),
        );
        let mut state = CoreState { keyval };

        let response = lpop(&mut state, &key);

        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::List(
                vec![
                    RespData::SimpleStr("second".into()),
                    RespData::SimpleStr("should_be_last".into()),
                ]
                .into()
            ))
        );
        assert_eq!(response, RespData::SimpleStr("first".into()));
    }

    #[test]
    fn returns_nil_when_the_list_is_empty() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::List(vec![].into()));
        let mut state = CoreState { keyval };

        let response = lpop(&mut state, &key);

        assert_eq!(state.keyval.get(&key), Some(&RespData::List(vec![].into())));
        assert_eq!(response, RespData::nil());
    }

    #[test]
    fn error_when_the_list_is_a_different_type() {
        let key: String = "key:00".into();

        let mut keyval = HashMap::new();
        keyval.insert(key.clone(), RespData::SimpleStr("not_a_list".into()));
        let mut state = CoreState { keyval };

        let response = lpop(&mut state, &key);

        // Assert that the original item was not mutated
        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::SimpleStr("not_a_list".into()))
        );
        assert_eq!(response, RespData::wrong_type());
    }
}

#[cfg(test)]
mod llen_should {
    use super::*;
    use crate::CoreState;
    use std::collections::HashMap;

    #[test]
    fn it_returns_the_length_of_a_list() {
        let mut keyval = HashMap::new();

        keyval.insert(
            "key".into(),
            RespData::List(
                vec![
                    RespData::Number(1),
                    RespData::Number(2),
                    RespData::Number(3),
                    RespData::Number(4),
                ]
                .into(),
            ),
        );

        let state = CoreState { keyval };

        let response = llen(&state, &"key".into());

        assert_eq!(response, RespData::Number(4));
    }

    #[test]
    fn it_returns_nil_when_the_list_isnt_there() {}
}
