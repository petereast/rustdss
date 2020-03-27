use super::CoreState;
use transport::RespData;

pub fn set(state: &mut CoreState, key: String, value: RespData) -> RespData {
    state.keyval.insert(key, value);
    RespData::ok()
}

pub fn get(state: &CoreState, key: String) -> RespData {
    state.keyval.get(&key).unwrap_or(&RespData::nil()).clone()
}
