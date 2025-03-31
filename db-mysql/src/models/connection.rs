/// MySQL数据库的连接设置
struct ConnectionSettings {
    /// 已保存连接的连接名称
    name: Option<String>,
    /// 数据库服务器主机（IP或主机名）
    host: String,
    /// 数据库服务器端口
    port: u16,
    /// 用于认证的用户名
    username: String,
    /// 用于认证的密码
    password: String,
    /// 要连接的可选数据库名称
    database: Option<String>,
    /// 连接超时（秒）
    timeout: u16,
}

/// 连接状态指示
enum ConnectionStatus {
    Disconnected,   // 已断开
    Connecting,     // 连接中
    Connected,      // 已连接
    Failed(String), // 失败（包含错误消息）
}

/// 已保存连接的集合
struct SavedConnections {
    connections: Vec<SavedConnection>,
}

/// 单个已保存的连接配置
struct SavedConnection {
    id: String,
    name: String,
    settings: ConnectionSettings,
}