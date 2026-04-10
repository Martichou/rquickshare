use tokio::net::TcpStream;
use async_trait::async_trait;
use anyhow::Result;
use tokio::io::AsyncWriteExt;

pub mod framing;

#[derive(Debug)]
pub struct TcpLanTransport
{
    pub socket: TcpStream
}

#[async_trait]
pub trait TransportMedium: std::fmt::Debug + Send + Sync + Unpin{
    async fn send_frame(&mut self, data: &[u8]) -> Result<()>;
    
    async fn recv_frame_header(&mut self) -> Result<usize>; 
    
    async fn recv_frame_payload(&mut self, length: usize) -> Result<Vec<u8>>;
    
    async fn disconnect(&mut self) -> Result<()>;
}

#[async_trait]
impl TransportMedium for TcpLanTransport{
    
    async fn send_frame(&mut self, data: &[u8]) -> Result<()> {
        framing::write_frame(&mut self.socket, data).await?;
        Ok(())
    }

    async fn recv_frame_header(&mut self) -> Result<usize> {
        let length = framing::read_frame_header(&mut self.socket).await?;
        Ok(length)
    }

    async fn recv_frame_payload(&mut self, length: usize) -> Result<Vec<u8>> {
        framing::read_frame_payload(&mut self.socket, length).await
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.socket.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_transport_integration() -> Result<()> {
        // Setup a local server to simulate the "other" phone
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        // Connect to that server using a real TcpStream
        let socket = TcpStream::connect(addr).await?;
        
        let mut transport = TcpLanTransport { socket };

        // Simulate the "other side" sending a message
        // [Header: 4 bytes (Length = 5)] + [Payload: 5 bytes ("Hello")]
        let mut server_side = listener.accept().await?.0;
        tokio::spawn(async move {
            let mut message = Vec::new();
            message.extend_from_slice(&5u32.to_be_bytes());
            message.extend_from_slice(b"Hello");          
            server_side.write_all(&message).await.unwrap();
        });

        // TEST: Receiving the data through the trait
        let header_len = transport.recv_frame_header().await?;
        assert_eq!(header_len, 5, "The header length should be 5 bytes");

        let payload = transport.recv_frame_payload(header_len).await?;
        assert_eq!(payload, b"Hello", "The payload should match 'Hello'");

        // TEST: Sending the data through the trait
        transport.send_frame(b"World").await?;

        Ok(())
    }
}