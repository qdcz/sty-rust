// 导入需要使用的模块
use std::collections::HashMap; // HashMap模块，用于管理客户端连接
use std::sync::{Arc, Mutex}; // Arc/Mutex模块，用于实现线程安全的共享变量
use tokio::net::{TcpListener, TcpStream}; // Tokio网络模块，用于处理TCP连接
use tokio::sync::broadcast; // Tokio同步模块，用于广播消息
use tokio::sync::mpsc::{channel, Sender, Receiver}; // Tokio同步模块，用于实现消息通道
use tokio::sync::oneshot; // Tokio同步模块，用于发送单个消息
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Tokio IO模块，用于异步读写数据
// use futures_util::{SinkExt, StreamExt}; // 引入futures_util中的SinkExt和StreamExt

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9001"; // 监听的地址和端口号
    let listener = TcpListener::bind(addr).await?; // 创建TCP监听器，并绑定到指定地址上
    println!("Listening on: {}", addr); // 打印监听的地址和端口号

    let (tx, mut rx) = channel(32); // 创建一个消息通道，用于从客户端接收消息并广播给其他客户端
    let clients = Arc::new(Mutex::new(HashMap::new())); // 创建一个线程安全的HashMap，用于管理连接的客户端

    let (tx_shutdown, rx_shutdown) = oneshot::channel(); // 创建一个单向消息通道，用于向任务发送关闭信号
    let (tx_error, rx_error) = broadcast::channel(32); // 创建一个广播通道，用于发送错误消息给所有连接的客户端

    // 启动一个任务来接收来自客户端的消息并将其广播给所有连接的客户端
    let broadcast_task = tokio::spawn(async move {
        loop {
            let msg = match rx.recv().await {   // 从消息队列接收消息
                Some(msg) => msg,              // 如果有消息，则进行下一步操作
                None => break,                 // 如果没有消息，则退出循环
            };
            let mut clients = clients.lock().unwrap();  // 获取客户端列表的互斥锁
            for (addr, client) in clients.iter_mut() {   // 遍历所有连接的客户端
                if let Err(_) = client.send(msg.clone()).await {  // 向客户端发送消息
                    println!("Error sending message to {}: {:?}", addr, msg);  // 如果发送失败，则打印错误信息
                }
            }
        }
    });

    // 启动一个任务来接收来自客户端的连接请求
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                tx_error.send(e.to_string()).unwrap();
                continue;
            }
        };
        let tx = tx.clone();
        let mut clients = clients.clone();
        let tx_shutdown = tx_shutdown.clone();
        let (client_tx, mut client_rx) = channel(32);

        // Add the client to the list of connected clients
        let mut clients = clients.lock().unwrap();
        clients.insert(addr, client_tx);

        // Spawn a task to handle messages from this client
        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            loop {
                let mut buf = vec![0; 4096];
                match reader.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let msg = buf[..n].to_vec();
                        tx.send(msg).await.unwrap();
                    }
                    Ok(_) => {
                        // Connection was closed
                        clients.remove(&addr);
                        break;
                    }
                    Err(e) => {
                        tx_error.send(e.to_string()).unwrap();
                        clients.remove(&addr);
                        break;
                    }
                }
            }
            tx_shutdown.send(()).unwrap();
        });
    }
}