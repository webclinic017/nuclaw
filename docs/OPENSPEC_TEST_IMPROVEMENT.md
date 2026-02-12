# OpenSpec 提案 - 低覆盖率模块测试改进

## 执行摘要

针对测试覆盖率低于 50% 的三个核心模块（task_scheduler.rs 16.8%、telegram.rs 12.9%、whatsapp.rs 6.7%），本提案提出通过**提取可测试逻辑**、**重构依赖隔离**和**补充边界测试**来提升覆盖率，同时遵循 KISS 原则，保持代码整洁。

## 问题分析

### 当前低覆盖率原因

| 模块 | 覆盖率 | 主要问题 |
|------|--------|----------|
| task_scheduler.rs | 16.8% | 大量异步数据库操作、容器执行依赖 |
| telegram.rs | 12.9% | HTTP API 调用、Webhook 服务器启动 |
| whatsapp.rs | 6.7% | MCP 服务依赖、网络轮询逻辑 |
| main.rs | 0% | CLI 入口、难以单元测试 |

### 不可测试代码特征

1. **外部服务依赖**：HTTP API、数据库、容器运行时
2. **异步 IO 操作**：网络请求、文件系统、定时器
3. **全局状态**：环境变量、静态配置
4. **副作用操作**：服务器启动、消息发送

## 解决方案

### 方案一：提取可测试逻辑（KISS）

将**纯逻辑**从**副作用代码**中分离：

```rust
// 可测试的纯逻辑
pub fn calculate_next_run(schedule_type: &str, value: &str) -> Option<String>
pub fn should_process_message(msg: &NewMessage, router: &RouterState) -> bool
pub fn chunk_text(text: &str, limit: usize) -> Vec<String>
```

### 方案二：Trait 抽象依赖（低耦合）

```rust
#[async_trait]
pub trait MessageService {
    async fn send(&self, jid: &str, content: &str) -> Result<()>;
}

// 生产实现
pub struct HttpMessageService;

// 测试实现
pub struct MockMessageService;
```

### 方案三：补充边界测试

对已提取的逻辑补充 100% 边界测试。

## 实施计划

### 阶段一：Task Scheduler（提升到 60%+）

| 任务 | 描述 | 预期覆盖率提升 |
|------|------|---------------|
| 1.1 | 提取 `calculate_next_run` 为纯函数 | +15% |
| 1.2 | 提取任务过滤逻辑 | +10% |
| 1.3 | 补充边界测试（无效 cron、空任务等） | +15% |
| 1.4 | 提取时间计算工具函数 | +5% |

**目标覆盖率**: 60%+

### 阶段二：Telegram（提升到 50%+）

| 任务 | 描述 | 预期覆盖率提升 |
|------|------|---------------|
| 2.1 | 提取消息分块逻辑为纯函数 | +10% |
| 2.2 | 提取策略检查逻辑 | +10% |
| 2.3 | 补充 Telegram 结构体序列化测试 | +10% |
| 2.4 | 提取 URL 构建和验证逻辑 | +5% |

**目标覆盖率**: 50%+

### 阶段三：WhatsApp（提升到 50%+）

| 任务 | 描述 | 预期覆盖率提升 |
|------|------|---------------|
| 3.1 | 提取消息去重逻辑 | +15% |
| 3.2 | 提取触发词检测逻辑 | +15% |
| 3.3 | 补充消息截断测试 | +10% |
| 3.4 | 提取组文件夹查找逻辑 | +5% |

**目标覆盖率**: 50%+

### 阶段四：Main（可选，提升到 30%+）

| 任务 | 描述 | 预期覆盖率提升 |
|------|------|---------------|
| 4.1 | 提取 CLI 参数验证逻辑 | +20% |
| 4.2 | 提取配置加载逻辑 | +10% |

**目标覆盖率**: 30%+

## 设计原则遵循

### 1. KISS（保持简单）

- 不做过度抽象
- 不引入复杂 mock 框架
- 优先提取纯函数而非 trait

### 2. 高内聚低耦合

```rust
// 内聚：相关逻辑在一起
pub mod message_logic {
    pub fn should_process(msg: &Message) -> bool
    pub fn extract_trigger(text: &str) -> Option<&str>
}

// 低耦合：通过参数传递依赖
pub async fn process_message<F>(msg: Message, sender: F) -> Result<()>
where F: Fn(&str) -> Future<Output = Result<()>>
```

### 3. 100% 测试率（新增代码）

所有提取的逻辑函数必须有：
- 正常路径测试
- 边界条件测试
- 错误处理测试

## 风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 重构引入 bug | 高 | 保持现有测试通过、小步提交 |
| 过度设计 | 中 | 严格遵循 KISS，拒绝复杂抽象 |
| 测试维护成本 | 低 | 纯函数测试简单稳定 |

## 测试策略

### 测试金字塔

```
    /\
   /  \  E2E (现有集成测试)
  /----\
 /      \  集成 (数据库测试)
/--------\
/          \ 单元 (新增重点)
------------
```

### 测试命令

```bash
# 运行新增单元测试
cargo test task_scheduler::
cargo test telegram::
cargo test whatsapp::

# 验证覆盖率
cargo tarpaulin --no-fail-fast --out Html -- --test-threads=1

# 确保无回归
cargo test --all
```

## 验收标准

- [ ] task_scheduler.rs 覆盖率 ≥ 60%
- [ ] telegram.rs 覆盖率 ≥ 50%
- [ ] whatsapp.rs 覆盖率 ≥ 50%
- [ ] main.rs 覆盖率 ≥ 30%（可选）
- [ ] 所有现有测试仍通过
- [ ] 代码通过 clippy 检查
- [ ] 新代码 100% 有测试

## 时间估算

| 阶段 | 预计时间 |
|------|---------|
| Task Scheduler | 2-3 小时 |
| Telegram | 2-3 小时 |
| WhatsApp | 2-3 小时 |
| Main | 1 小时 |
| 验证与文档 | 1 小时 |
| **总计** | **8-11 小时** |

---

**提案版本**: v1.0
**生成日期**: 2026-02-12
**状态**: 待实现
