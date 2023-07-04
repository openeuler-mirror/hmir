


#[cfg(test)]
mod tests {

    use std::process::{Command, Output};
    use tokio::process::Command as AsyncCommand;
    use tokio::io::AsyncReadExt;
    use tokio;
    use uuid::Uuid;
    use gethostname::gethostname;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng,Rng};
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[feature(unix_chown)]
    use std::os::unix::fs;

    extern crate machine_uid;




    fn ceph_generate_key() -> String {
        let output = Command::new("ceph-authtool").arg("--gen-print-key")
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
        let str_vec: Vec<&str> = binding.split(",").collect();


        let items: Vec<u32> = str_vec.iter().map(|s|s.parse::<u32>().unwrap()).collect();
        println!("{:?}",items);
        // .iter()
            // .filter_map(|s| s.parse().ok())
            // .collect();
        return (0,0);
        // return (items[0],items[1]);
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

    fn write_tmp(content: &String,uid:u32,gid:u32) -> std::io::Result<File> {
        let temp_dir = env::temp_dir();
        let mut file_path = temp_dir.clone();
        file_path.push(format!("{}{}","ceph-tmp",gernerate_random_letters()));
        let mut file =File::create(&file_path)?;
        fs::fchown(&file, Some(uid), Some(gid))?;
        file.write(content.as_ref());
        file.flush();
        Ok(file)
    }

    fn create_initial_keys() {
        let mon_key = ceph_generate_key();
        let admin_key = ceph_generate_key();
        let mgr_key = ceph_generate_key();
        let mgr_id = generate_service_id();
        let (uid, gid) = extract_uid_gid();

        let keyring = format!("[mon.]\n
         \tkey = {}\n
         \tcaps mon = allow *\n
         [client.admin]\n
         \tkey = {}\n
         \tcaps mon = allow *\n
         \tcaps mds = allow *\n
         \tcaps mgr = allow *\n
         \tcaps osd = allow *\n
         [mgr.{}]\n
         \tkey = {}\n
         \tcaps mon = profile mgr\n
         \tcaps mds = allow *\n
         \tcaps osd = allow *\n"
            ,mon_key, admin_key, mgr_id, mgr_key);

        let admin_keyring_context = format!("[client.admin]\n\tkey =  + {} + \n",admin_key);
        let admin_keyring = write_tmp(&admin_keyring_context,uid,gid);
        let bootstrap_keyring = write_tmp(&keyring, uid, gid);
    }

    fn create_initial_monmap() {

    }

    fn ceph_bootstrap() {
        let CEPH_CONFIG_PATH = "/etc/ceph/ceph.conf";
        let (uid, gid) = extract_uid_gid();
        create_initial_keys()
    }


    #[tokio::test]
    async fn create_ceph_cluster() {

        extract_uid_gid();
        // ceph_bootstrap();
    }

}