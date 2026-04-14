# CocoonBreak - 信息茧房破解工具

基于 Chromium 无头客户端的 Bilibili 信息茧房破解工具，使用 Tauri 作为 UI 框架。

## 功能特性

- 🌐 **无头浏览器**: 基于 Chromium 的无头客户端，自动访问 Bilibili
- 📊 **内容分析**: 使用 LLM 分析推荐内容，识别信息茧房倾向性
- 🔍 **反向搜索**: 智能生成反向搜索词，打破信息壁垒
- 🎯 **自动观看**: 自动搜索并观看多样化内容
- 🖥️ **现代 UI**: 基于 Tauri 的桌面应用界面

## 技术栈

- **后端**: Rust + Tauri
- **前端**: HTML + CSS + JavaScript
- **浏览器**: headless_chrome (Chromium)
- **AI 分析**: OpenAI GPT-4 API
- **数据抓取**: scraper + reqwest

## 安装依赖

### 前置要求

- Rust 1.70+
- Node.js 18+
- Chromium 浏览器

### 安装步骤

1. 克隆项目
```bash
git clone <repository-url>
cd CocoonBreak
```

2. 安装 Rust 依赖
```bash
cargo build
```

3. 配置 OpenAI API Key
在应用界面中输入你的 OpenAI API Key

## 使用方法

### 1. 启动应用

```bash
cargo tauri dev
```

### 2. 配置 API Key

在应用界面中输入你的 OpenAI API Key

### 3. 获取推荐内容

点击"获取推荐内容"按钮，系统会自动抓取 Bilibili 首页推荐

### 4. 分析内容

点击"分析内容"按钮，LLM 会分析推荐内容的倾向性

### 5. 生成反向搜索

点击"生成反向搜索"按钮，系统会生成打破信息茧房的搜索词

### 6. 开始破解

点击"开始破解茧房"按钮，系统会自动搜索并观看多样化内容

## 项目结构

```
CocoonBreak/
├── src/
│   ├── main.rs          # 应用入口
│   ├── lib.rs           # Tauri 应用配置
│   ├── models.rs        # 数据模型
│   ├── browser.rs       # 无头浏览器封装
│   ├── bilibili.rs      # Bilibili 抓取逻辑
│   ├── llm.rs           # LLM 集成
│   └── commands.rs      # Tauri 命令处理
├── dist/
│   ├── index.html       # 前端页面
│   ├── styles.css       # 样式文件
│   └── app.js           # 前端逻辑
├── Cargo.toml           # Rust 依赖配置
├── tauri.conf.json      # Tauri 配置
└── build.rs             # 构建脚本
```

## 工作原理

1. **内容抓取**: 使用无头 Chromium 浏览器访问 Bilibili 首页
2. **数据分析**: 将推荐内容发送给 LLM 进行分析
3. **茧房识别**: LLM 识别内容倾向性和茧房化程度
4. **反向搜索**: 生成与当前内容形成对比的搜索词
5. **内容多样化**: 自动搜索并观看不同类型的内容

## 注意事项

- 需要有效的 OpenAI API Key
- 首次运行需要下载 Chromium
- 建议在稳定的网络环境下使用
- 遵守 Bilibili 的使用条款

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！