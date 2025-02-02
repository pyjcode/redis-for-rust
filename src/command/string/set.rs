
use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{command_strategy::CommandStrategy, db::db::Redis, session::session::Session, RedisConfig};

/*
 * Set 命令
 */
pub struct SetCommand {}

impl CommandStrategy for SetCommand {
    fn execute(
        &self,
        stream: &mut TcpStream,
        fragments: &Vec<&str>,
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
    ) {
        let mut redis_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(&stream.peer_addr().unwrap().to_string()) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let value = fragments[6].to_string();
        if fragments.len() > 8 {
            if fragments[8].to_uppercase() == "PX" {
                let ttl = fragments[10].parse::<i64>().unwrap();
                redis_ref.set_with_ttl(db_index, key.clone(), value.clone(), ttl);
            } else if fragments[8].to_uppercase() == "EX" {
                let ttl = fragments[10].parse::<i64>().unwrap();
                let ttl_millis = ttl * 1000;
                redis_ref.set_with_ttl(db_index, key.clone(), value.clone(), ttl_millis);
            }
        } else {
            redis_ref.set(db_index, key, value);
        }
        stream.write(b"+OK\r\n").unwrap();
    }
}