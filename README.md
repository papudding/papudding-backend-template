# papudding-backend-template

一个基于Rust和Actix-web的后端模板项目，提供了完整的用户认证、路由管理和数据库交互功能，可作为快速开发后端服务的起点。

## 项目特性

- **Web框架**：使用Actix-web构建高性能异步HTTP服务
- **数据库交互**：集成Rbatis ORM框架，支持MySQL数据库
- **认证系统**：基于JWT的用户认证和授权机制
- **模块化设计**：清晰的代码组织结构，包括路由、处理器、服务和模型分层
- **配置管理**：使用TOML配置文件管理应用参数
- **日志系统**：完善的日志记录功能
- **错误处理**：统一的错误处理机制

## 技术栈

- **核心语言**：Rust 1.70+
- **Web框架**：actix-web 4
- **ORM**：rbatis 4.5
- **数据库**：MySQL
- **认证**：JWT (jsonwebtoken 9.2.0)
- **序列化**：serde, serde_json
- **日志**：log, fast_log, env_logger
- **配置**：toml, lazy_static

## 项目结构

```
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config.toml       # 应用配置文件
└── src/
    ├── config/       # 配置相关
    ├── dto/          # 数据传输对象
    ├── handler/      # 请求处理器
    ├── main.rs       # 应用入口
    ├── model/        # 数据模型
    ├── router/       # 路由定义
    ├── service/      # 业务逻辑
    └── util/         # 工具函数
```

## 安装与配置

### 前提条件

- Rust 1.70+ 开发环境
- MySQL 数据库
- Git

### 安装步骤

1. 克隆仓库

```bash
git clone https://github.com/yourusername/papudding-backend-template.git
cd papudding-backend-template
```

2. 配置数据库

编辑 `config.toml` 文件，设置数据库连接信息：

```toml
[database]
user = "root"
password = "your_password"
host = "localhost"
port = 3306
dbname = "your_database"

[jwt]
token_secret = "your_secret_key_here"
```

3. 构建项目

```bash
cargo build --release
```

## 使用方法

### 启动服务

```bash
cargo run --release
```

服务将在配置文件指定的地址和端口启动（默认通常为 `localhost:8080`）。

### API 接口文档

#### 认证接口

- **用户注册**
  - URL: `/auth/register`
  - 方法: POST
  - 请求体: 
    ```json
    {
      "username": "string",
      "password": "string",
      "email": "string"
    }
    ```
  - 响应: 用户信息

- **用户登录**
  - URL: `/auth/login`
  - 方法: POST
  - 请求体: 
    ```json
    {
      "username": "string",
      "password": "string"
    }
    ```
  - 响应: JWT 令牌

#### 测试接口

所有测试接口需要认证，前缀为 `/test`：

- `/test/echo` - 回显请求
- `/test/hello` - 简单问候
- `/test/test_db` - 数据库测试
- `/test/test_get_user_id` - 获取当前用户ID
- `/test/hey` - 手动问候接口

## 开发指南

### 项目扩展

1. 添加新路由：在 `src/router/` 目录下创建新的路由文件
2. 添加新处理器：在 `src/handler/` 目录下实现请求处理逻辑
3. 添加新服务：在 `src/service/` 目录下实现业务逻辑
4. 添加新数据模型：在 `src/model/` 目录下定义数据结构

### 运行测试

```bash
cargo test
```

## 许可证

[MIT](LICENSE)
        