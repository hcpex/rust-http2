use tokio::{ io::{ ReadHalf, AsyncReadExt }, net::TcpStream };
use tokio_boring::SslStream;

use crate::utils;

pub struct Http2Reader {
    stream: ReadHalf<SslStream<TcpStream>>,
}

impl Http2Reader {
    pub fn new(stream: ReadHalf<SslStream<TcpStream>>) -> Self {
        Http2Reader { stream }
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, std::io::Error> {
        // self.stream.read(buf).await?;
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp = vec![0u8; 1024];

        loop {
            let n = self.stream.read(&mut tmp).await?;
            if n == 0 {
                return Err(
                    std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Connection closed")
                );
            }

            buffer.extend_from_slice(&tmp[..n]);

            while buffer.len() >= 9 {
                let payload_len = utils::extract_length(&buffer).await;
                let total_len = 9 + (payload_len as usize);

                if buffer.len() < total_len {
                    break;
                }

                let frame = buffer[..total_len].to_vec();
                // println!("Received complete frame of length: {}", frame.len());
                // println!("Frame: {:?}", frame);

                buffer.drain(..total_len);

                return Ok(frame);
            }
        }
    }
}
