//  引入标准库下的 IO 包, Net 网络包, Thread包,String 工具包以及Time 工具包
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

// param TcpStream, handle client request, return
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 一个用来存储的数组
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // read 读取信息
        let bytes_read = stream.read(&mut buf)?;
        // 传入数据为空, 成功返回
        if bytes_read == 0 {
            return Ok(());
        }

        // 使用match 来进行错误的处理
        let s = match str::from_utf8(&buf[..bytes_read]) {
            // 如果转换成功返回字符串值。
            Ok(v) => v,
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(e) => {
                // 输出调试信息。
                stream.write(b"Need utf-8 sequence.").unwrap();
                // 继续监听，虽然本次输入的字符流格式不是utf8 字符，但是不影响下次输入所以不需要 panic!
                continue;
            }
        };
        println!("get client request {}", s);

        // 在输入流中写入读取到的数据
        stream.write(&buf[..bytes_read])?;

        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 在localhost 监听8080端口号, 处理请求
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // 使用 Vec 来存储线程
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // accept connections and process them serially
    for stream in listener.incoming() {
        // 异常处理 没有拿到 抛出failed
        let stream = stream.expect("failed!");

        // handle_client 处理,拿到JoinHandle 的返回值类型
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // thread_vec 加入线程
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // 使用join方法,等待子线程执行结束,unwrap() 表示不处理错误，遇到错误直接出错退出程序。
        handle.join().unwrap();
    }
    // 执行完成
    Ok(())
}
