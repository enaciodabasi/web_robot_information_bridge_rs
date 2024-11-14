pub mod robot_info_msg;

use std::{error::Error};

use config::{Config, File};
use std::path::Path;
use tokio::{
    io::AsyncReadExt,
    sync::{mpsc}
};

const MAX_BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let web_bridge_config = Config::builder()
        .add_source(File::from(Path::new("config/web_bridge_config.yaml")))
        .build();

    let web_bridge_config = match web_bridge_config {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading web_bridge_config.toml: {}", e);
            return Err(Box::from(e));
        }
    };

    let ip_addr = web_bridge_config.get::<String>("ip_addres");
    let ip_addr: String = match ip_addr {
        Ok(ip_addr) => ip_addr,
        Err(e) => {
            eprintln!("Error loading ip_addres from web_bridge_config.toml: {}", e);
            return Err(Box::from(e));
        }
    };

    let port = web_bridge_config.get::<u16>("port");
    let port: u16 = match port {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Error loading port from web_bridge_config.toml: {}", e);
            return Err(Box::from(e));
        }
    };

    let socket_addr: core::net::SocketAddr =
        format!("{}:{}", ip_addr, port).as_str().parse().unwrap();

    let socket = tokio::net::TcpSocket::new_v4()?;
    socket.bind(socket_addr)?;

    let listener = socket.listen(1024);
    let listener = match listener {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error binding to socket: {}", e);
            return Err(Box::from(e));
        }
    };

    let (tx, _rx) = mpsc::channel::<robot_info_msg::Data>(32);

    //let mut command_dequeue: std::collections::VecDeque::<robot_info_msg::Data> = VecDeque::new();
    
    loop {
        let (mut sock_stream, _) = listener.accept().await?;
        let tx_c = tx.clone();
        tokio::spawn(async move {
            let mut buf = [0; MAX_BUFFER_SIZE];
            loop {
                let n = sock_stream.read(&mut buf).await.unwrap();
                if n == 0 {
                    return;
                }
                let msg = String::from_utf8_lossy(&buf[..n]);
                let msg = serde_json::from_str::<robot_info_msg::Data>(&msg);
                let _msg = match msg {
                    Ok(msg) => {
                      tx_c.send(msg).await.unwrap();
                    },
                    Err(e) => {
                        eprintln!("Error parsing message: {}", e);
                        return;
                    }
                };
                
            }
        });
    }

    Ok(())
}
