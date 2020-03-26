// This will provide list support - eventually support blocking commands?
// Commands: LPUSH, RPUSH, LPOP, RPOP, RPOPLPUSH

use super::{CoreState, Key};
use crate::transport::RespData;

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
