use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error};

#[tauri::command]
pub fn cmd_login(host : & str, port : i32 , username : & str, password : & str) -> usize
{
    const USE_SSH_LOGIN : bool = false;
    if ws_client_mgr::register_client(host,port) {
        if USE_SSH_LOGIN {
            return login_mgr::ssh_login(host,username,password);
        }
        return login_mgr::login(host,username,password);
    } else {
        error!("Can't register clinet : {}:{}",host,port);
        return errno::HMIR_ERR_CONNECT_SERVER;
    }
}

#[tauri::command]
pub fn cmd_logout(host : &str) -> bool
{
    return login_mgr::logout(host);
}