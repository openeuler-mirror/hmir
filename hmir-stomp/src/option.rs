use std::fs::File;
use std::io;
use std::io::BufReader;
use std::sync::Arc;
use anyhow::{anyhow, Error};
use clap::{Arg, App};

use rustls::{Certificate, PrivateKey};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{self},
};
use crate::error::StompError;


pub struct ServerOption {
    pub(crate) ip: Option<String>,
    pub(crate) port: Option<String>,
    /// 公开的证书公钥文件
    pub(crate) cert: Option<String>,
    /// 隐私的证书私钥文件
    pub(crate) key: Option<String>,

}

impl ServerOption {

    fn load_certs(path: &Option<String>) -> anyhow::Result<Vec<Certificate>> {
        if let Some(path) = path {
            let file = File::open(path).expect(format!("cannot open cert file: {}", path).as_str());
            let mut reader = BufReader::new(file);
            let certs = rustls_pemfile::certs(&mut reader)?;
            Ok(certs.into_iter().map(Certificate).collect())
        } else {
            return Err(anyhow!(StompError::FileReadError));
        }
    }

    fn load_keys(path: &Option<String>) -> anyhow::Result<PrivateKey> {
        if let Some(path) = path {
            let file = File::open(path).expect(format!("cannot open key file: {}", path).as_str());
            let mut reader = BufReader::new(file);
            let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
            match keys.len() {
                0 => Err(anyhow!(StompError::FileReadError)),
                1 => Ok(PrivateKey(keys.remove(0))),
                _ => Err(anyhow!(StompError::FileReadError)),
            }
        } else {
            return Err(anyhow!(StompError::FileReadError));
        }
    }

    pub async fn get_tls_accept(&mut self) -> anyhow::Result<TlsAcceptor> {


        let certs = Self::load_certs(&self.cert)?;
        let key = Self::load_keys(&self.key)?;

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        let acceptor: TlsAcceptor = TlsAcceptor::from(Arc::new(config));
        Ok(acceptor)
    }


    pub fn parse_option() -> anyhow::Result<ServerOption> {

        let mut app = App::new("rvdsm");
        let matches = app.clone()
            .version("1.2.1")
            .author("duanwujie")
            .about("vdsm in rust")
            .arg(Arg::with_name("ip")
                .short('b')
                .long("bind")
                .takes_value(true)
                .help("监听地址"))
            .arg(Arg::with_name("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .help("监听端口"))
            .arg(Arg::with_name("cert")
                .short('c')
                .long("cert")
                .takes_value(true)
                .help("证书的公钥"))
            .arg(Arg::with_name("key")
                .short('k')
                .long("key")
                .takes_value(true)
                .help("证书的私钥"))
            .arg(Arg::with_name("tls")
                .long("tls")
                .takes_value(true)
                .help("证书的私钥"))
            .get_matches();

        let mut ip = matches.value_of("ip").or(Some("0.0.0.0"));
        let mut port = matches.value_of("port").or(Some("54321"));
        let mut cert = matches.value_of("cert").or(Some("/etc/pki/vdsm/certs/vdsmcert.pem"));
        let mut key = matches.value_of("key").or(Some("/etc/pki/vdsm/keys/vdsmkey.pem"));


        Ok(ServerOption {
            cert: cert.map(|s|s.to_string()),
            key: key.map(|s|s.to_string()),
            ip: ip.map(|s|s.to_string()),
            port : port.map(|s|s.to_string()),
        })

    }
}

