use super::CoreState;
use crate::transport::RespData;

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
