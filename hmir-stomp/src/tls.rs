use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::sync::Arc;
use std::net::{
    SocketAddr,
    ToSocketAddrs,
};
use rustls::{Certificate, PrivateKey, RootCertStore, server::{AllowAnyAuthenticatedClient}};
use tokio::net::{TcpStream};
use tokio_rustls::{
    TlsAcceptor,
    TlsConnector,
    rustls::{self},
    client::TlsStream as ClientTlsStream,
};




pub struct TlsOption {
    /// 公开的证书公钥文件
    pub(crate) cert: String,
    /// 隐私的证书私钥文件
    pub(crate) key: String,
}

impl TlsOption {
    fn load_certs(path: &str) -> io::Result<Vec<Certificate>> {
        let file = File::open(path).expect(format!("cannot open cert file: {}",path).as_str());
        let mut reader = BufReader::new(file);
        let certs = rustls_pemfile::certs(&mut reader)?;
        Ok(certs.into_iter().map(Certificate).collect())
    }

    fn load_keys(path: &str) -> io::Result<PrivateKey> {
        let file = File::open(&path).expect(format!("cannot open key file: {}",path).as_str());
        let mut reader = BufReader::new(file);
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
        match keys.len() {
            0 => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("No PKCS8-encoded private key found"),
            )),
            1 => Ok(PrivateKey(keys.remove(0))),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("More than one PKCS8-encoded private key found"),
            )),
        }
    }

    pub async fn get_tls_accept(&mut self) -> anyhow::Result<TlsAcceptor> {

        let certs = Self::load_certs(&self.cert)?;
        let key = Self::load_keys(&self.key)?;


        // let config = rustls::ServerConfig::builder()
        //     .with_cipher_suites(rustls::DEFAULT_CIPHER_SUITES)
        //     .with_safe_default_kx_groups()
        //     .with_protocol_versions(rustls::DEFAULT_VERSIONS)
        //     .expect("inconsitant cipher-suits/versions specified")
        //
        //     .with_no_client_auth()
        //     .with_single_cert(certs, key)
        //     .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        let acceptor: TlsAcceptor = TlsAcceptor::from(Arc::new(config));
        Ok(acceptor)
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};

    const CA_FILE: &str = "cert/dev/ca.cert";
    const CLIENT_CERT_FILE: &str = "cert/dev/client.cert";
    const CLIENT_KEY_FILE: &str = "cert/dev/client.key";

    const SERVER_CERT_FILE: &str = "/etc/pki/vdsm/certs/vdsmcert.pem";
    const SERVER_KEY_FILE: &str = "/etc/pki/vdsm/keys/vdsmkey.pem";

    #[tokio::test]
    async fn tls() -> anyhow::Result<()>{





    }

    // async fn start_server() {
    //     let tls_acceptor = new_tls_acceptor(SERVER_CERT_FILE, SERVER_KEY_FILE);
    //     let listener = TcpListener::bind("0.0.0.0:54321").await.unwrap();
    //
    //     tokio::spawn(async move {
    //         let (stream, _peer_addr) = listener.accept().await.unwrap();
    //         let mut tls_stream = tls_acceptor.accept(stream).await.unwrap();
    //         println!("server: Accepted client conn with TLS");
    //
    //         let mut buf = [0; 12];
    //         tls_stream.read(&mut buf).await.unwrap();
    //         println!("server: got data: {:?}", buf);
    //         tls_stream.write(&buf).await.unwrap();
    //         println!("server: flush the data out");
    //     });
    // }
    //
    // async fn start_client(msg: &[u8], buf: &mut [u8]) {
    //     let addr = lookup_ipv4("127.0.0.1", 5002);
    //     let mut tls_stream =
    //         new_tls_stream("localhost", addr, CA_FILE, CLIENT_CERT_FILE, CLIENT_KEY_FILE).await;
    //
    //     tls_stream.write(msg).await.unwrap();
    //     println!("client: send data");
    //
    //     tls_stream.read(buf).await.unwrap();
    //     println!("client: read echoed data");
    // }
}