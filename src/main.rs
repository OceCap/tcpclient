/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-07-12 23:36:34
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-07-13 21:53:44
 * @FilePath: \tcp\tcpclient\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<( )> {
    //创建流连接服务端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..1000 {
        //输入
        let mut input = String::new();
        //控制台读入一行到input中
        io::stdin().read_line(&mut input).expect("Failed to read");
        //输入的内容转换为bytes写入流中
        stream.write(input.as_bytes()).expect("Failed to write");
        //从stream创建一个reader，读取服务端写入的内容
        let mut reader = BufReader::new(&stream);
        //创建vec缓冲区
        let mut buff: Vec<u8> = Vec::new();
        //从reader中读取内容到缓冲区
        reader.read_until(b'\n', &mut buff).expect("Failed to read to buff");
        //打印内容
        println!("read from tcpserver: {:?}", str::from_utf8(&buff).unwrap());
    }
    Ok(())
}

