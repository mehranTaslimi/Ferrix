use tokio::net::TcpListener;

use super::*;

impl Registry {
    pub(super) async fn initialize_peer_listener() -> anyhow::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:6881").await?;

        println!("Peer listener started on");

        spawn!("peer_listener", {
            loop {
                let (socket, addr) = listener.accept().await.unwrap();
                spawn!("-> peer_listener", {
                    println!("Handling peer connection from: {}", addr);
                });
            }
        });

        Ok(())
    }
}
