use crate::RunMode;

/// 解析是哪种机器
/// 没解析出来会 panic，panic 就是你没看 stderr 输出的东西
pub fn parse_run_mode() -> RunMode {
    let args = std::env::args().collect::<Vec<String>>();
    
    // 检查是否有两个玩意在一起
    if (
        args.contains(&"--sender".to_string()) || args.contains(&"-s".to_string())
    ) && (
        args.contains(&"--receiver".to_string()) || args.contains(&"-r".to_string())
    ) {
        panic!("请只指定一个运行模式, 运行参数 --sender/-s 或 --receiver/-r")
    }

    if args.contains(&"--sender".to_string()) || args.contains(&"-s".to_string()) {
        RunMode::Sender
    } else if args.contains(&"--receiver".to_string()) || args.contains(&"-r".to_string()) {
        RunMode::Receiver
    } else {
        panic!("请指定运行模式, 运行参数 --sender/-s 或 --receiver/-r")
    }
}

pub fn parse_config_path() -> Option<String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.contains(&"--config".to_string()) || args.contains(&"-c".to_string()) {
        let index = args.iter().position(|x| x == &"--config".to_string() || x == &"-c".to_string()).unwrap();
        Some(args[index + 1].clone())
    } else {
        None
    }
}
