use crate::CoreState;
use rustdss_data::RespData;
use rustdss_transport::serialise::SerialiseRespData;

pub fn flushall(state: &mut CoreState) -> RespData {
    state.keyval.clear();
    RespData::ok()
}

pub fn keys(state: &CoreState) -> RespData {
    RespData::List(
        state
            .keyval
            .keys()
            .map(|key| RespData::SimpleStr(key.into()))
            .collect(),
    )
}

pub fn dump(state: &CoreState, key: &String) -> RespData {
    state
        .keyval
        .get(key)
        .map(|value| RespData::BulkStr(value.as_string()))
        .unwrap_or(RespData::nil())
}
