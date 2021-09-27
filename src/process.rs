use std::{borrow::BorrowMut, collections::HashMap, net::SocketAddr, sync::{Arc, Mutex, MutexGuard}};

use crate::user;

use super::user::User;

pub(crate) fn process_result(msg: String, addr: SocketAddr, users_arc: &Arc<Mutex<HashMap<SocketAddr, User>>>) -> String {
    let users_mutex = match users_arc.lock() {
        Ok(x) => x,
        _ => return format!("error while accessing mutex\n"),
    };
    let nickname = match users_mutex.get(&addr) {
        Some(x) => x.nickname.clone(),
        None => format!("{}", addr),
    };

    let (prefix, msg) = match msg.trim_end().split_once(':') {
        Some(x) => x,
        None => return format!("no prefix for the command\n"),
    };

    match prefix {
        "say" => return format!("{} says : {}\n", nickname, msg),
        "nick" => return change_nickname(msg.to_string(), addr, users_mutex),
        _ => return format!("prefix unknown\n"),
    };
}

fn change_nickname(msg: String, addr: SocketAddr, mut users_mutex: MutexGuard<HashMap<SocketAddr, User>>) -> String {
    match users_mutex.get_mut(&addr) {
        Some(x) => {x.change_nickname(msg.clone());},
        None => {users_mutex.insert(addr, User::new(msg.clone()));},
    };
    return format!("{} changed their name for {}\n", addr, msg);
}