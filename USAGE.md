# CocoonBreak 使用指南

## 快速开始

### 1. 环境准备

确保你的系统已安装以下软件：
- Rust (1.70 或更高版本)
- Node.js (18 或更高版本)
- Chromium 浏览器（会自动下载）

### 2. 获取 API Key

你需要一个 OpenAI API Key 来使用 AI 分析功能：
1. 访问 https://platform.openai.com/
2. 注册或登录账户
3. 在 API Keys 页面创建新的 API Key
4. 复制 API Key 保存好

### 3. 配置项目

1. 复制环境变量示例文件：
```bash
cp .env.example .env
```

2. 编辑 `.env` 文件，填入你的 API Key：
```
OPENAI_API_KEY=sk-your-actual-api-key-here
```

### 4. 构建和运行

#### 开发模式
```bash
cargo tauri dev
```

#### 生产构建
```bash
cargo tauri build
```

构建完成后，可执行文件位于 `src-tauri/target/release/` 目录

## 功能使用说明

### 第一步：获取推荐内容

1. 在应用界面的 "API 配置" 区域输入你的 OpenAI API Key
2. 点击 "获取推荐内容" 按钮
3. 系统会自动启动无头浏览器访问 Bilibili 首页
4. 抓取推荐视频并显示在界面上

### 第二步：分析内容

1. 点击 "分析内容" 按钮
2. LLM 会分析推荐视频的内容
3. 显示分析结果，包括：
   - 内容总结
   - 主要话题标签
   - 偏见分析
   - 信息茧房评分（0-100%）

### 第三步：生成反向搜索

1. 点击 "生成反向搜索" 按钮
2. LLM 会根据内容分析生成反向搜索策略
3. 显示：
   - 反向搜索词列表
   - 生成理由
   - 预期效果

### 第四步：开始破解茧房

1. 点击 "开始破解茧房" 按钮
2. 系统会自动：
   - 依次搜索反向搜索词
   - 显示搜索进度和找到的视频
   - 自动观看找到的视频
3. 完成后会显示完成提示

## 信息茧房评分说明

- **0-30%** 🟢：信息获取较为均衡，茧房化程度较低
- **30-60%** 🟡：存在一定程度的茧房化，建议多样化内容
- **60-80%** 🟠：茧房化程度较高，需要主动打破信息壁垒
- **80-100%** 🔴：严重茧房化，强烈建议使用反向搜索功能

## 高级配置

### 使用其他 LLM 服务

如果你想使用其他兼容 OpenAI API 的服务（如 Azure OpenAI、Anthropic 等），可以修改 `.env` 文件：

```
OPENAI_BASE_URL=https://your-custom-endpoint.com/v1
OPENAI_MODEL=your-model-name
```

### 自定义 Chromium 路径

如果需要使用特定版本的 Chromium，可以在 `.env` 中指定：

```
CHROMIUM_PATH=/path/to/your/chromium
```

### Bilibili Cookie（可选）

如果需要访问登录后的内容，可以添加 Bilibili Cookie：

1. 在浏览器中登录 Bilibili
2. 打开开发者工具 (F12)
3. 在 Application/Storage 中找到 Cookie
4. 复制 Cookie 值到 `.env` 文件：
```
BILIBILI_COOKIE=your-cookie-here
```

## 故障排除

### 构建失败

如果遇到构建错误，尝试：
```bash
cargo clean
cargo build
```

### Chromium 下载失败

如果 Chromium 下载失败，可以手动下载并指定路径：
1. 从 https://www.chromium.org/ 下载对应版本
2. 在 `.env` 中设置 `CHROMIUM_PATH`

### API 调用失败

检查：
1. API Key 是否正确
2. 网络连接是否正常
3. API 额度是否充足
4. 是否需要配置代理

### Bilibili 抓取失败

可能原因：
1. 网络连接问题
2. Bilibili 页面结构变化
3. 被反爬虫机制拦截

解决方案：
1. 检查网络连接
2. 添加 Cookie 认证
3. 增加请求间隔时间

## 最佳实践

1. **定期使用**：建议每周使用 1-2 次，持续改善信息获取多样性
2. **结合手动探索**：自动工具辅助，手动探索为主
3. **关注评分变化**：记录茧房评分变化，评估改善效果
4. **调整搜索策略**：根据效果调整反向搜索词生成策略
5. **保护隐私**：不要在不信任的网络环境中使用

## 注意事项

1. **API 费用**：使用 OpenAI API 会产生费用，请注意控制使用量
2. **网络流量**：自动观看视频会消耗网络流量
3. **合规使用**：遵守 Bilibili 的使用条款和 robots.txt
4. **数据隐私**：不要在公共环境中暴露 API Key
5. **适度使用**：避免过度依赖自动化工具

## 技术支持

如遇到问题：
1. 查看日志文件了解详细错误信息
2. 检查 GitHub Issues 是否有类似问题
3. 提交新的 Issue 并附上错误日志

## 贡献指南

欢迎贡献代码！请：
1. Fork 项目
2. 创建特性分支
3. 提交 Pull Request
4. 等待代码审查

## 许可证

MIT License - 详见 LICENSE 文件