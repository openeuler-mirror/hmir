
use std::process::{Command, ExitStatusError, Output};
use tokio::process::Command as AsyncCommand;
use tokio::io::AsyncReadExt;
use tokio;
use uuid::Uuid;
use gethostname::gethostname;
use rand::distributions::Alphanumeric;
use rand::{thread_rng,Rng};
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::os::fd::{AsFd, AsRawFd};
use std::path::{Path, PathBuf};

#[feature(unix_chown)]
use std::os::unix::fs as unix_fs;
use nix::libc::daemon;

extern crate machine_uid;

const DATA_DIR: &str = "/var/lib/ceph";
const DATA_DIR_MODE: u32 = 0o700;
const LOG_DIR: &str ="/var/log/ceph";

fn ceph_hostname() -> String {
    return gethostname().into_string().unwrap();
}

fn ceph_generate_key() -> String {
    let output = Command::new("ceph-authtool")
        .arg("--gen-print-key")
        .output()
        .expect("failed to execute process");
    let binding = String::from_utf8_lossy(&output.stdout);
    return binding.to_string();
}

async fn ceph_version_() -> Result<Output, std::io::Error> {
    let mut child = AsyncCommand::new("ceph").arg("--version").stdout(std::process::Stdio::piped()).spawn()?;
    let output = child.wait_with_output().await?;
    Ok(output)
}

fn make_fsid() -> String {
    let ns = Uuid::from_u128(0);
    let host_id: String = machine_uid::get().unwrap();
    let name = host_id.to_string();

    let uuid = Uuid::new_v4();
    uuid.to_string()
}

fn extract_uid_gid() -> (u32,u32) {
    let output = Command::new("stat")
        .arg("-c")
        .arg("%u,%g")
        .arg("/var/lib/ceph")
        .output()
        .expect("failed to execute process");
    let binding = String::from_utf8_lossy(&output.stdout);
    let binding = binding.trim();
    let str_vec: Vec<&str> = binding.split(",").collect();
    let items: Vec<u32> = str_vec
        .iter()
        .map(|s|s.parse::<u32>().unwrap())
        .collect();
    return (items[0],items[1]);
}

async fn deploy_daemon(daemon_type : String) {

    let daemon_id = "mon_id";
    let fsid = "fsid";
    let config_path = "/var/lib/ceph";
    let keyring_path = "/tmp/keyring";

    // let mut child = AsyncCommand::new("/usr/bin/ceph-mon").arg("--mkfs")
    //     .arg("-i").arg(daemon_id)
    //     .arg("--fsid").arg(fsid)
    //     .arg("-c").arg(config_path)
    //     .arg("--keyring").arg(keyring_path)
    //     .stdout(std::process::Stdio::piped()).spawn()?;


}

fn gernerate_random_letters() -> String {
    let charset= b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = thread_rng();
    let random_chars = (0..6).map(|_|{
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    }).collect();
    return random_chars;
}

fn generate_service_id() -> String {
    return gethostname().into_string().unwrap() + "." + &gernerate_random_letters();
}

fn write_tmp(content: &String, uid:u32, gid:u32) -> std::io::Result<String> {
    let temp_dir = env::temp_dir();
    let mut file_path = temp_dir.clone();
    file_path.push(format!("{}{}","ceph-tmp",gernerate_random_letters()));
    let mut file =fs::File::create(&file_path)?;
    file.write_all(content.as_ref());
    file.flush();
    unix_fs::fchown(&file, Some(uid), Some(gid))?;
    Ok(file_path.to_string_lossy().to_string())
}

fn delete_file(file_path: PathBuf) -> std::io::Result<()>
{
    fs::remove_file(file_path)?;
    Ok(())
}

fn delete_dir(file_path: PathBuf) -> std::io::Result<()>
{
    fs::remove_dir_all(file_path)?;
    Ok(())
}

fn init_env(mon_id : &String)
{
    let temp_dir = env::temp_dir();
    let mut file_path = temp_dir.join("keyring");
    delete_file(file_path);
    let mon_dir = get_data_dir(&DATA_DIR.to_string(), &"mon".to_string(), mon_id);
    delete_dir(mon_dir);

}

fn write_tmp_keyring(content: &String, uid:u32, gid:u32) -> std::io::Result<String> {
    let temp_dir = env::temp_dir();
    let mut file_path = temp_dir.join("keyring");
    let mut file =fs::File::create(&file_path);
    match file {
        Ok(mut f) => {
            f.write_all(content.as_ref());
            f.flush();
            unix_fs::fchown(&f, Some(uid), Some(gid)).unwrap();
        }
        Err(_) => {println!("Create /tmp/keyring failed")}
    }
    Ok(file_path.to_string_lossy().to_string())
}

fn create_initial_keys(uid:u32,gid:u32,mgr_id:&str) ->(String,String,String,String,String) {
    println!("========================Begin to create the initial keys ======================");
    let mon_key = ceph_generate_key();
    let admin_key = ceph_generate_key();
    let mgr_key = ceph_generate_key();

    let keyring = format!("[mon.]\n\
    \tkey = {}\n\
    \tcaps mon = allow *\n\
    [client.admin]\n\
    \tkey = {}\n\
    \tcaps mon = allow *\n\
    \tcaps mds = allow *\n\
    \tcaps mgr = allow *\n\
    \tcaps osd = allow *\n\
    [mgr.{}]\n\
    \tkey = {}\n\
    \tcaps mon = profile mgr\n\
    \tcaps mds = allow *\n\
    \tcaps osd = allow *\n"
        ,mon_key, admin_key, mgr_id, mgr_key);

    let admin_keyring_context = format!("[client.admin]\n\tkey =  + {} + \n",admin_key);
    let admin_keyring = write_tmp(&admin_keyring_context,uid,gid);
    let bootstrap_keyring = write_tmp_keyring(&keyring, uid, gid);



    (mon_key,mgr_key,admin_key,bootstrap_keyring.unwrap(),admin_keyring.unwrap())
}


fn create_initial_monmap(uid:u32,gid:u32,fsid : &str,mon_id: &str,mon_addr: &str) {
    println!("========================Begin to initial the monmap======================");
    // let monmap = write_tmp(&"".to_string(), 0, 0);
    let output = Command::new("/usr/bin/monmaptool")
        .arg("--create")
        .arg("--clobber")
        .arg("--fsid").arg(fsid)
        .arg("--addv").arg(mon_id).arg(mon_addr)
        .arg("/tmp/monmap")
        .output()
        .expect("failed to execute monmaptool process");
    println!("{}",String::from_utf8_lossy(&output.stdout));
    let result = unix_fs::chown("/tmp/monmap", Some(uid), Some(gid));
    match result {
        Ok(()) => { println!("chown /tmp/monmap to ceph.ceph")}
        _ => {println!("chown /tmp/monmap : {}",&Error::last_os_error());}
    }
}



fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    return path.exists();
}




fn change_permission(path: &str,mode:u32) -> Result<(),std::io::Error>{
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path);
    let mut perms = metadata?.permissions();
    perms.set_mode(mode);
    fs::set_permissions(path,perms)?;
    Ok(())
}


fn makedirs(dir:&String, uid:u32, gid:u32, mode:u32) {
    if file_exists(dir.as_str()) {
        fs::create_dir_all(dir);
        println!("mkdir : {}",dir);
    } else {
        println!("chmod : {}",dir);
        change_permission(&dir.as_str(), mode);
        unix_fs::chown(&dir, Some(uid), Some(gid));
        change_permission(&dir.as_str(), mode);
    }
}


fn make_data_dir_base(fsid: &String, data_dir: &String, uid:u32, gid:u32) {
    let path = PathBuf::from(data_dir);
    let data_dir_base = path.join(fsid);
    let crash_path_base = data_dir_base.join("crash");
    let posted_path_base = crash_path_base.join("posted");
    makedirs(&data_dir_base.to_string_lossy().to_string(), uid, gid, DATA_DIR_MODE);
    makedirs(&crash_path_base.to_string_lossy().to_string(), uid, gid, DATA_DIR_MODE);
    makedirs(&posted_path_base.to_string_lossy().to_string(),uid, gid, DATA_DIR_MODE);
    // return data_dir_base
}

fn get_data_dir(data_dir : &String,daemon_type:&String,daemon_id:&String) -> PathBuf {
    let path = PathBuf::from(data_dir);
    let daemon_path = path.join(daemon_type);
    let suffix = "ceph-".to_string() + daemon_id;
    let data_base = daemon_path.join(suffix);
    return data_base;
}


fn get_log_dir(fsid : &String,log_dir : &String) -> PathBuf {
    let log_base = PathBuf::from(log_dir);
    return log_base;
}

// fn make_data_dir(fsid: &String, daemon_type: &String, daemon_id: &String, uid:u32, gid:u32) -> String  {
//     make_data_dir_base(fsid, &DATA_DIR.to_string(),uid, gid);
//     let data_dir = get_data_dir( &DATA_DIR.to_string(), daemon_type, daemon_id);
//     makedirs(&data_dir, uid, gid, DATA_DIR_MODE);
//     return data_dir;
// }

fn create_daemon_dirs(fsid: &String, daemon_type: &String, daemon_id: &String, uid:u32, gid:u32) {
    // let data_dir = make_data_dir(fsid, daemon_type, daemon_id, uid, gid);


}


fn prepare_create_mon(uid:u32,gid:u32,fsid:&String,mon_id:&String,bootstrap_keyring_path:&String,monmap_path:&String)
{
    // create_daemon_dirs(fsid, &"mon".to_string(), mon_id, uid, gid);
    let mon_dir = get_data_dir( &DATA_DIR.to_string(), &"mon".to_string(), mon_id);
    println!("mon_dir is: {}",mon_dir.to_string_lossy().to_string());
    let log_dir = get_log_dir(fsid, &LOG_DIR.to_string());
    println!("log_dir is: {}",log_dir.to_string_lossy().to_string());
    println!("========================Begin to initial the mon======================");
    let output = Command::new("/usr/bin/ceph-mon")
        .arg("--mkfs")
        .arg("-i").arg(mon_id)
        .arg("--fsid").arg(fsid)
        // .arg("-c").arg("/dev/null")
        .arg("--monmap").arg("/tmp/monmap")
        .arg("--keyring").arg("/tmp/keyring")
        .output()
        .expect("failed to execute ceph-mon process");

    println!("{}",String::from_utf8_lossy(&output.stdout));
    println!("{}",String::from_utf8_lossy(&output.stderr));

}

fn get_unit_file(data_dir:&String,daemon_type:&String,daemon_id:&String,) -> String {

    let unit = format!("# generated by rustcephadm\n\
    [Unit]
    Description=Ceph %i for {}\n\

    # According to:\n\
    #   http://www.freedesktop.org/wiki/Software/systemd/NetworkTarget\n\
    # these can be removed once ceph-mon will dynamically change network\n\
    # configuration.\n\
        After=network-online.target local-fs.target time-sync.target\n\
    Wants=network-online.target local-fs.target time-sync.target\n\

    PartOf=ceph-{}.target\n\
    Before=ceph-{}.target\n\
    \n\
    [Service]\n\
    LimitNOFILE=1048576\n\
    LimitNPROC=1048576\n\
    EnvironmentFile=-/etc/environment\n\
    ExecStart=/bin/bash {}/{}/ceph-{}/unit.run\n\
    ExecStop=-/bin/bash {}/{}/ceph-{}/unit.stop\n\
    ExecStopPost=-/bin/bash {}/{}/ceph-{}/unit.poststop\n\
    KillMode=none\n\
    Restart=on-failure\n\
    RestartSec=10s\n\
    TimeoutStartSec=120\n\
    TimeoutStopSec=120\n\
    StartLimitInterval=30min\n\
    StartLimitBurst=5\n\
    \n\
    [Install]\n\
    WantedBy=ceph-{}.target\n",daemon_id,
                       daemon_type,
                       daemon_type
                       ,data_dir,daemon_type,daemon_id
                       ,data_dir,daemon_type,daemon_id
                       ,data_dir,daemon_type,daemon_id
                       ,daemon_type);
    return unit;
}

// fn prepare_mon_addresses() -> (String,bool,String) {
//     //cephadm bootstrap --mon-ip 172.16.88.13
//
//     let ipv6 = false;
//     let addrv_args = [];
//
// }

pub fn command_bootstrap() {



    let CEPH_CONFIG_PATH = "/etc/ceph/ceph.conf";
    let (uid, gid) = extract_uid_gid();
    let mgr_id = generate_service_id();
    let fsid = make_fsid();
    let mon_id = ceph_hostname();
    let addr_arg="v1:172.30.21.13:6789";

    init_env(&mon_id);

    let  (mon_key,mgr_key,admin_key,bootstrap_keyring,admin_keyring) =
        create_initial_keys(uid,gid,mgr_id.as_str());
    println!("mon_key : {}",&mon_key);
    println!("mgr_key : {}",&mgr_key);
    println!("admin_key : {}",&admin_key);
    println!("bootstrap_keyring : {}",&bootstrap_keyring);
    println!("admin_keyring : {}",&admin_keyring);


    create_initial_monmap(uid,gid,fsid.as_str(),mon_id.as_str(),addr_arg);

    prepare_create_mon(uid,gid,&fsid,&mon_id,&bootstrap_keyring,&"/tmp/monmap".to_string())
}





#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn create_ceph_cluster() {

        let daemon_id = "dwj";
        let daemon_type = "mon";
        let data_dir = "/var/lib/ceph/";
        let unit = get_unit_file(&data_dir.to_string(),&daemon_type.to_string(),&daemon_id.to_string());
        println!("{}",unit);
    }
}