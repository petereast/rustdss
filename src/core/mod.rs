use crate::request::command::{Command, Key};
use crate::transport::RespData;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

// Exists for the lifetime of the application.
pub type Message = (Command, SyncSender<RespData>);
// This is the stateful part of the application
pub struct Core {
    sender: SyncSender<Message>,
}

struct CoreState {
    keyval: HashMap<Key, RespData>,
}

fn core_logic(state: &mut CoreState, cmd: Command) -> RespData {
    let response = match cmd {
        Command::Set(key, value) => {
            state.keyval.insert(key, value);
            RespData::ok()
        }
        Command::Get(key) => state
            .keyval
            .get(&key)
            .unwrap_or(&RespData::Error("nil".into()))
            .clone(),
        Command::FlushAll => {
            state.keyval.clear();
            RespData::ok()
        }
        Command::Incr(key, maybe_by) => {
            let prev = state.keyval.get(&key);

            let op = match prev {
                Some(RespData::Number(val)) => Ok(RespData::Number(val + maybe_by.unwrap_or(1))),
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
        _ => RespData::Error("Unknown core cmd".into()),
    };
    response
}

impl Core {
    pub fn start() -> Self {
        // Could do something interesting using a threadpool - key-hash sharding for example
        println!("[core] starting core");
        let (sender, reciever) = sync_channel::<Message>(50);

        thread::spawn(move || {
            let mut state: CoreState = CoreState {
                keyval: HashMap::new(),
            };
            loop {
                if let Ok(msg) = reciever.recv() {
                    let (cmd, responder) = msg;

                    let response = core_logic(&mut state, cmd);
                    responder
                        .send(response)
                        .expect("[core] can't reply to messages");
                } else {
                    println!("[core] death");
                }
            }
        });
        Self { sender: sender }
    }

    pub fn get_sender(&self) -> SyncSender<Message> {
        self.sender.clone()
    }
}
#[cfg(test)]
mod test {
    use super::*;

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
}
