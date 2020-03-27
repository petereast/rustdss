use super::CoreState;
use crate::transport::RespData;

pub fn incr(state: &mut CoreState, key: String, maybe_by: Option<i64>) -> RespData {
    let prev = state.keyval.get(&key);

    let op = match prev {
        Some(RespData::Number(val)) => Ok(RespData::Number((*val) + maybe_by.unwrap_or(1))),
        Some(_) => Err(RespData::Error("NaN".into())),
        None => Ok(RespData::Number(1)),
    };

    if let Ok(new_val) = op {
        state.keyval.insert(key, new_val.clone());
        new_val
    } else {
        op.err().unwrap()
    }
}

pub fn decr(state: &mut CoreState, key: String, maybe_by: Option<i64>) -> RespData {
    let prev = state.keyval.get(&key);

    let op = match prev {
        Some(RespData::Number(val)) => Ok(RespData::Number((*val) - maybe_by.unwrap_or(1))),
        Some(_) => Err(RespData::Error("NaN".into())),
        None => Ok(RespData::Number(-1)),
    };

    if let Ok(new_val) = op {
        state.keyval.insert(key, new_val.clone());
        new_val
    } else {
        op.err().unwrap()
    }
}
