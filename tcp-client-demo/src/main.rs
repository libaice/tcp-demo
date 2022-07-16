use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    // 连接服务端 8080 端口
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        // 读取命令行的输入
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        // 向TcpStream的流中来写入数据
        stream.write(input.as_bytes()).expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        // buffer 来读取命令行中写入的数据
        let mut buffer: Vec<u8> = Vec::new();

        //Read all bytes into buf until the delimiter byte or EOF is reached. 直到结束前, 一直读取数据
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        // 将读取到的数据，进行打印输出
        println!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("  =============  ")
    }
    Ok(())
}


// Example  https://doc.rust-lang.org/std/io/struct.Stdin.html
/*
    use std::io;

    fn main() -> io::Result<()> {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        Ok(())
}

*/
