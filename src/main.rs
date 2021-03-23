use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;

fn main() {
    println!("Starting TcpServer ...");

    // 1. 绑定本地监听服务
    let listener = TcpListener::bind("127.0.0.1:80").expect("Unable to bind to socket");
    // 2. 获取本地地址
    let addr = listener.local_addr().expect("Unable to get the local port");
    // 3. 控制台打印监听端口
    println!("listening the port: {}", addr.port());
    // 4. 循环获取连接客户端
    for connection in listener.incoming() {
        // 4.1 使用match分支模式以及Err捕获模式
        match connection {
            Ok(stream) => {
                // 4.2 开启线程并调用客户端消息处理逻辑
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // 1. 初始化链接时回传客户端，表示连接成功
    if let Err(_) = stream.write("echo skyhuihui".as_bytes()){
        return;
    }
    println!("client connected");
    // 2. 读取客户端发送消息
    let mut buff = [0; 16];
    loop {
        // 2.1 读取客户端消息至buff
        if let Ok(read) = stream.read(&mut buff) {
            if read == 0 {
                break;
            }
            // 2.2 将buff消息打印到控制台
            if let Ok(msg) = std::str::from_utf8(&buff[0..read]) {
                println!("{}", msg);
                // 2.3 将buff消息回传给客户端
                if let Err(_) = stream.write(&buff[0..read]) {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    // 3. 断开服务连接
    println!("client disconnected");
}