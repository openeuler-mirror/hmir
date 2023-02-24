//! 软件包管理模块
//!
//! 支持的系统
//! - debian系
//! - euler系
//! 支持以下的请求
//! -
//!
use std::process;
use jsonrpsee::ws_server::{RpcModule};
use std::str::Split;
use regex::Regex;
use hmir_protocol::pkg;
use std::io::{BufReader, Read};
use std::fs::File;
use hmir_hash::HashWrap;

use hmir_errno::errno;
use hmir_token::TokenChecker;


// use hmir_dpkg;

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("pkg-all-info", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(pkg_all_info())
    })?;
    Ok(())
}

pub fn pkg_all_info() -> String{
    return dpkg_package_info();
}


fn dpkg_package_info() -> String
{
    let package_re = Regex::new(r"^Package: (.*)$").unwrap();
    let status_re = Regex::new(r"^Status: (.*)$").unwrap();
    let prio_re = Regex::new(r"^Priority: (.*)$").unwrap();
    let section_re = Regex::new(r"^Section: (.*)$").unwrap();
    let isize_re = Regex::new(r"^Installed-Size: (.*)$").unwrap();
    let maintain_re = Regex::new(r"^Maintainer: (.*)$").unwrap();
    let arch_re = Regex::new(r"^Architecture: (.*)$").unwrap();
    let source_re = Regex::new(r"^Source: (.*)$").unwrap();
    let version_re = Regex::new(r"^Version: (.*)$").unwrap();
    let depends_re = Regex::new(r"^Depends: (.*)$").unwrap();
    let description_re = Regex::new(r"^Description: (.*)$").unwrap();

    let mut package = "";
    let mut status = "";
    let mut priority = "";
    let mut section = "";
    let mut installed_size = 0;;
    let mut maintainer = "";
    let mut architecture = "";
    let mut source = "";
    let mut version = "";
    let mut depends = vec!["".to_string()];
    let mut description = "";

    // let packagedb = std::str::from_utf8(&output.stdout);
    let mut packagedb = String::new();
    let file = File::open("/var/lib/dpkg/status").expect("Error in reading file");
    let mut bufferReader = BufReader::new(file);
    bufferReader.read_to_string(&mut packagedb).expect("Unable to read line");
    let mut map  = hmir_hash::HashWrap::new();

    let records: Split<&str> = packagedb.split("\n\n");
    for record in records {
        let rlines: Split<&str> = record.split("\n");
        for line in rlines {
            if package_re.is_match(line) {
                package = line.trim_start_matches("Package: ");
            }
            else if status_re.is_match(line) {
                status = line.trim_start_matches("Status: ");
            }
            else if prio_re.is_match(line) {
                priority = line.trim_start_matches("Priority: ");
            }
            else if section_re.is_match(line) {
                section = line.trim_start_matches("Section: ");
            }
            else if isize_re.is_match(line) {
                installed_size = line.trim_start_matches("Installed-Size: ").parse().unwrap_or(0);
            }
            else if maintain_re.is_match(line) {
                maintainer = line.trim_start_matches("Maintainer: ");
            }
            else if arch_re.is_match(line) {
                architecture = line.trim_start_matches("Architecture: ");
            }
            else if source_re.is_match(line) {
                source = line.trim_start_matches("Source: ");
            }
            else if version_re.is_match(line) {
                version = line.trim_start_matches("Version: ");
            }
            else if depends_re.is_match(line) {
                let depend = line.trim_start_matches("Depends: ");
                depends = record.split(",").into_iter().map(|x|x.to_string()).collect();
            }
            else if description_re.is_match(line) {
                description = line.trim_start_matches("Description: ");
            }
        }

        let p = pkg::DebPkgInfo {
            package: package.to_string(),
            status: status.to_string(),
            priority: priority.to_string(),
            section: section.to_string(),
            installed_size: installed_size,
            maintainer: maintainer.to_string(),
            architecture: architecture.to_string(),
            source: source.to_string(),
            version: version.to_string(),
            depends: depends.clone(),
            description: description.to_string(),
        };
        map.insert(package,p);
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    // fn dpkg_list_it_worked(){
    //     hmir_dpkg::dpkg_list();
    // }

}