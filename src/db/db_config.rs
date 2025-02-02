use std::env;

/*
 * Redis 配置
 * 
 * @param host 地址
 * @param port 端口
 * @param databases 初始化数据库 
 * @param password 密码
 */
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub databases: usize,
    pub aof_file_path: Option<String>
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            databases: get_databases_or(16),
            port: get_port_or(6379),
            password: get_password_or(None),
            aof_file_path: get_aof_file_path_or(None)
        }
    }
}

/*
 * 获取 port 参数
 *
 * @param default 默认端口（6379）
 */
fn get_port_or(default: u16) -> u16 {
    let mut args = env::args().skip_while(|arg| arg != "--port").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--port' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 databases 参数
 *
 * @param default 默认数量（16）
 */
fn get_databases_or(default: usize) -> usize {
    let mut args = env::args().skip_while(|arg| arg != "--databases").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--databases' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 password 参数
 *
 * @param default_password 默认密码（None）
 */
fn get_password_or(default_password: Option<String>) -> Option<String> {
    let mut args = env::args().skip_while(|arg| arg != "--password").take(2);
    if args.next().is_none() {
        return default_password;
    }

    if let Some(arg) = args.next() {
        return Some(arg);
    } else {
        return default_password;
    }
}

/*
 * 获取持久化文件路径参数
 */
fn get_aof_file_path_or(default_aof_file_path: Option<String>) -> Option<String> {
    let mut args = env::args().skip_while(|arg| arg != "--aof_file_path").take(2);
    if args.next().is_none() {
        return default_aof_file_path;
    }

    if let Some(arg) = args.next() {
        return Some(arg);
    } else {
        return default_aof_file_path;
    }
}