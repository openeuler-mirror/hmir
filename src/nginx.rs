//! 该模块是空模块，只提供文档。通过配置nginx反向代理,完成安全配置
//!
//! 下面是nginx的参考配置: /etc/nginx/conf.d/websocket.conf
//! ```
//!upstream websocket {
//!    server 127.0.0.1:8080; # appserver_ip:ws_port
//!}

//!server {
//!    server_name 172.30.21.13;
//!    listen 443;
//!    location / {
//!        proxy_pass http://websocket;
//!        proxy_read_timeout 300s;
//!        proxy_send_timeout 300s;
//!
//!         proxy_set_header Host $host;
//!         proxy_set_header X-Real-IP $remote_addr;
//!         proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
//!
//!        proxy_http_version 1.1;
//!         proxy_set_header Upgrade $http_upgrade;
//!         proxy_set_header Connection $connection_upgrade;
//!     }
//!
//!     ssl on;
//!     ssl_certificate /etc/hmir/cert.pem;
//!     ssl_certificate_key /etc/hmir/key.pem;
//! }
//! ```
//!
//! ```
//! systemctl nginx restart
//! ```
//!
//! 其中 127.0.0.1:8080 是hmir监听的端口
