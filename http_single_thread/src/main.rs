// 引入读写IO库
use std::io::{Read, Write};
// 引入网络库TcpListener和TcpStream
use std::net::{TcpListener, TcpStream};

fn main() {
    // 引入网络库TcpListener和TcpStream
    let listener = TcpListener::bind("127.0.0.1:5329").unwrap();
    // 打印监听的地址和端口号
    println!("Listening on 127.0.0.1:5329...");

    // 循环监听传入的连接
    for stream in listener.incoming() {
        // 获取TcpStream
        let mut stream = stream.unwrap();
        // 定义缓冲区，长度为1024字节
        let mut buffer = [0; 1024];
        // 读取客户端传来的数据
        stream.read(&mut buffer).unwrap();

        // 构造HTTP响应内容
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        // 将响应内容写回到TcpStream
        stream.write(response.as_bytes()).unwrap();
        // 刷新缓冲区，确保响应已经发送到客户端
        stream.flush().unwrap();
    }
}
