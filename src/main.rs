fn main() {
    // M007：Rust 默认忽略 SIGPIPE，管道读者早退（如 `reader … | head`）时 println! 会
    // panic（exit 101）且喷 stderr；恢复默认处置后按 Unix 惯例被信号静默终止（同
    // grep/rg，shell 报 141）。Windows 无 SIGPIPE，不适用。
    #[cfg(unix)]
    {
        // SAFETY: 进程启动最早点、单线程环境下调用，仅恢复 SIGPIPE 默认处置，
        // 不与其它信号处理器交互；返回值（原处置）无需使用。
        unsafe { libc::signal(libc::SIGPIPE, libc::SIG_DFL) };
    }
    std::process::exit(reader_rs::run());
}
