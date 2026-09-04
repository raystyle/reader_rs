//! mirror 模块公开 API 回归（G006 回归层独立 target）：`download_file` 落盘前自建父目录。
//! 场景即 ISSUE #1 大陆侧验收回执的 bug（M017）：`ocr init` 对不存在的缓存目录树
//! 三通道下载全败 `os error 3`（`fs::write` 对缺失父目录的形态）；修复为下载器不依赖
//! 调用方建目录。独立 target 专用的原因：公开 API 测试进 `tests\`（G005），而
//! `READER_MIRROR` 只能进程内改 env，混编 target（cli.rs 等）里会串染并行 assert_cmd
//! 子进程的继承 env，本二进制只有本用例，变更自包含。

use reader_rs::mirror::{download_file, FilePin, PackagePin, Source};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::TcpListener;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// 一次性 HTTP 服务：接受一个连接，读完请求头后回固定 200 与 `body`。
/// 读到请求头结束再回写，避免关连接时残留未读数据触发 RST 截掉响应。
fn serve_once(listener: TcpListener, body: &'static [u8]) {
    let (mut stream, _) = listener.accept().expect("接受连接");
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .expect("设读超时");
    let mut seen = Vec::new();
    let mut buf = [0u8; 1024];
    while !seen.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut buf).expect("读请求头");
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&buf[..n]);
    }
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).expect("写响应头");
    stream.write_all(body).expect("写响应体");
}

/// `download_file` 对不存在的缓存目录树自建父目录后落盘（M017 回归）：
/// 合成 pin 指向本机一次性 HTTP 服务（`READER_MIRROR` 覆盖），dest 深埋
/// 全不存在的目录层级；修复前此处为「三通道全败： 写临时件失败: os error 3」。
#[test]
fn download_file_creates_missing_parent_dirs() -> TestResult {
    let body: &'static [u8] = b"abc";
    let sha: &'static str = Box::leak(format!("{:x}", Sha256::digest(body)).into_boxed_str());
    let file: &'static FilePin = Box::leak(
        FilePin {
            name: "model.safetensors",
            bytes: body.len() as u64,
            sha256: sha,
        }
        .into(),
    );
    let pin: &'static PackagePin = Box::leak(
        PackagePin {
            name: "syn-pkg",
            repo: "x/y",
            revision: "0123456789abcdef0123456789abcdef01234567",
            files: std::slice::from_ref(file),
        }
        .into(),
    );

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let server = std::thread::spawn(move || serve_once(listener, body));

    let case = std::env::temp_dir().join(format!("reader-mirror-dl-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&case);
    let dest = case
        .join("models")
        .join("syn-pkg")
        .join("model.safetensors");

    std::env::set_var("READER_MIRROR", format!("http://127.0.0.1:{port}"));
    let outcome = download_file(pin, file, &dest);
    std::env::remove_var("READER_MIRROR");
    server.join().expect("服务线程");

    assert_eq!(outcome.expect("下载并落盘"), Source::Mirror);
    assert_eq!(std::fs::read(&dest).expect("件应已落盘"), body);
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}
