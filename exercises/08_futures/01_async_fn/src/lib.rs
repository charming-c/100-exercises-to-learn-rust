use tokio::net::TcpListener;
extern crate rand;

use rand::Rng;
// TODO: write an echo server that accepts incoming TCP connections and
//  echoes the received data back to the client.
//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
// pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
//     // loop {
//     //     let (mut socket, addr) = listener.accept().await?;
//     //     // let mut buf = [0u8; 1024];
    
//     //     println!("get connection from: {:?}", addr);
//     //     let (mut reader, mut writer) = socket.split();
//     //     let n = copy(&mut reader, &mut writer).await?;
//     //     if n == 0 {
//     //         return Ok(())
//     //     }
//     // }

//     loop {
//         println!("laalalala");
//         let (mut socket, _) = listener.accept().await?;
//         let (mut reader, mut writer) = socket.split();
//         tokio::io::copy(&mut reader, &mut writer).await?;
//     }
   
// }

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, addr) = listener.accept().await?;
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start 
        // accepting new connections
        tokio::spawn(async move {
            println!("connection from {:?}", addr);
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello";100];
        let mut rng = rand::thread_rng();

        for request in requests {
            let n = rng.gen::<u16>();
            sleep(Duration::from_micros(n as u64));
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            println!("connection start... addr:{:?}", &socket.local_addr().unwrap());
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
