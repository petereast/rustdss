use super::{Command, CoreState};
use crate::transport::RespData;

use super::key_val;
use super::number;

fn flushall(state: &mut CoreState) -> RespData {
    state.keyval.clear();
    RespData::ok()
}

// Maybe move this mapping function into the module root?
pub fn core_logic(state: &mut CoreState, cmd: Command) -> RespData {
    let response = match cmd {
        Command::Set(key, value) => key_val::set(state, key, value),
        Command::Get(key) => key_val::get(state, key),
        Command::FlushAll => flushall(state),
        Command::Incr(key, maybe_by) => number::incr(state, key, maybe_by),
        Command::Decr(key, maybe_by) => number::decr(state, key, maybe_by),
        _ => RespData::Error("Unknown core cmd".into()),
    };
    response
}
#[cfg(test)]
mod should {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn set_adds_a_new_key() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = core_logic(
            &mut state,
            Command::Set("a".into(), RespData::SimpleStr("hello".into())),
        );

        assert_eq!(response, RespData::ok());
        assert_eq!(state.keyval.len(), 1);
        assert_eq!(
            state.keyval.get("a"),
            Some(&RespData::SimpleStr("hello".into()))
        );
    }

    #[test]
    fn get_gets_a_key() {
        let mut inner_keyval = HashMap::new();
        inner_keyval.insert("a".into(), RespData::SimpleStr("hello".into()));

        let mut state = CoreState {
            keyval: inner_keyval,
        };

        let response = core_logic(&mut state, Command::Get("a".into()));

        assert_eq!(response, RespData::SimpleStr("hello".into()));
        assert_eq!(state.keyval.len(), 1);
        assert_eq!(
            state.keyval.get("a"),
            Some(&RespData::SimpleStr("hello".into()))
        );
    }

    #[test]
    fn get_returns_nil_when_key_is_not_found() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let response = core_logic(&mut state, Command::Get("a".into()));

        assert_eq!(response, RespData::NullString);
        assert_eq!(state.keyval.len(), 0);
        assert_eq!(state.keyval.get("a"), None);
    }

    #[test]
    fn set_overwrites_existing_value() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        let key: String = "key-a".into();

        let response_a = core_logic(
            &mut state,
            Command::Set(key.clone(), RespData::SimpleStr("hello".into())),
        );

        let response_b = core_logic(
            &mut state,
            Command::Set(key.clone(), RespData::SimpleStr("goodbye".into())),
        );

        assert_eq!(response_a, RespData::ok());
        assert_eq!(response_b, RespData::ok());
        assert_eq!(state.keyval.len(), 1);
        assert_eq!(
            state.keyval.get(&key),
            Some(&RespData::SimpleStr("goodbye".into()))
        );
    }

    #[test]
    fn flushall_deletes_everything() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        core_logic(
            &mut state,
            Command::Set("a".into(), RespData::SimpleStr("hello".into())),
        );
        core_logic(
            &mut state,
            Command::Set("b".into(), RespData::SimpleStr("goodbye".into())),
        );

        assert_eq!(state.keyval.len(), 2);
        assert_eq!(
            state.keyval.get("a"),
            Some(&RespData::SimpleStr("hello".into()))
        );
        assert_eq!(
            state.keyval.get("b"),
            Some(&RespData::SimpleStr("goodbye".into()))
        );

        core_logic(&mut state, Command::FlushAll);

        assert_eq!(state.keyval.len(), 0);
        assert_eq!(state.keyval.get("a"), None);
        assert_eq!(state.keyval.get("b"), None);
    }

    #[test]
    fn incr() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        // It creates a key when there isn't one
        let response = core_logic(&mut state, Command::Incr("a".into(), None));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(1)));
        assert_eq!(response, RespData::Number(1));

        // It increments existing keys
        let response = core_logic(&mut state, Command::Incr("a".into(), None));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(2)));
        assert_eq!(response, RespData::Number(2));

        // It increments by the given amount
        let response = core_logic(&mut state, Command::Incr("a".into(), Some(10)));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(12)));
        assert_eq!(response, RespData::Number(12));
    }

    #[test]
    fn decr() {
        let mut state = CoreState {
            keyval: HashMap::new(),
        };

        // It creates a key when there isn't one
        let response = core_logic(&mut state, Command::Decr("a".into(), None));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(-1)));
        assert_eq!(response, RespData::Number(-1));

        // It decrements existing keys
        let response = core_logic(&mut state, Command::Decr("a".into(), None));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(-2)));
        assert_eq!(response, RespData::Number(-2));

        // It decrements by the given amount
        let response = core_logic(&mut state, Command::Decr("a".into(), Some(10)));
        assert_eq!(state.keyval.get("a"), Some(&RespData::Number(-12)));
        assert_eq!(response, RespData::Number(-12));
    }
}
