---
document_id: '7643718224996158900'
directory_id: '7641931416103620022'
title: Agent 回复云文档评论
full_path: /mcp_open_tools/feishu-cli/use-cases/agent-replies-to-doc-comments
breadcrumb:
- Lark CLI
- Agent best practices
- Agent replies to doc comments
document_type: GuideDocumentType
updated_at: 2026-05-29T07:11:41Z
source_url: https://open.larksuite.com/document/mcp_open_tools/feishu-cli/use-cases/agent-replies-to-doc-comments
---

# 智能体回复云文档评论

本文档提供Lark云文档 Bot 评论回复功能的完整配置指南。配置完成后，Bot 即可在文档评论区响应 @ 提及，并自动进行回复，适用于文档答疑、信息整理、任务分派等场景。

> 💡  不想手动配置？ 把这篇文档的链接发给你的智能体（OpenClaw / Hermes / 自建 bot 等），让它读完按里面的步骤来——它会自己识别你的部署类型，按对应选项一步步引导你完成。


## 一、功能说明

Bot 要能响应评论，需同时满足以下条件：

| # | 条件        | 说明  | 本手册覆盖 |
| -- | --------- | ------------ | ----- |
| 1  | 事件订阅已开通   | Lark开放平台已为 bot 订阅`drive.notice.comment_add_v1` 事件 | ✅ 第二节 |
| 2  | 权限已开通     | bot 已申请评论读写等相关权限并通过审核                            | ✅ 第二节 |
| 3  | 应用已发布     | 权限变更后应用已重新发布上线                                   | ✅ 第二节 |
| 4  | Agent 已运行 | bot 背后的 Agent 部署完成，可接收并处理该事件                     | ✅ 第三节 |
| 5  | 文档已授权     | 具体文档已将访问权限授予该 bot                                | ✅ 第四节 |

> **注：** 条件 1–5 均需同时满足。Bot 的业务逻辑（如何回复、回复什么内容）由各框架自己的 Agent 实现，不在本手册范围内。

## 二、开放平台配置（所有部署类型通用）

无论你用的是 OpenClaw、Hermes 还是自建 bot，本节 4 步完全一致。
打开[Lark开放平台](https://open.larksuite.com/)，找到你的 bot 应用，依次完成 Step 1–4。

> 给 Agent 的提示：如果你是 Agent 在执行本节配置，可以跳到本节末尾的「附录：（Agent 专用）一键配置链接」，用一条 URL 代替 Step 1–4 的手工操作。

### Step 1：订阅评论事件

进入「**事件与回调** → **添加事件**」，搜索并添加：
```
drive.notice.comment_add_v1
```

### Step 2：将事件模式切换为长连接

在「**事件配置**」和「**回调配置**」中，确认模式均为**长连接**。
> ⚠️ 如果当前是 webhook URL 模式，请切换为长连接。本功能不支持 HTTP 回调模式。

### Step 3：添加评论相关权限

进入「**权限管理** → **批量导入**」，把以下 JSON 粘贴到 `scopes.tenant` 中：

```
{
  "scopes": {
    "tenant": [
      "drive:drive.metadata:readonly",
      "docs:document.comment:create",
      "docs:document.comment:delete",
      "docs:document.comment:read",
      "docs:document.comment:update",
      "docs:document.comment:write_only",
      "docx:document:create",
      "docx:document:readonly",
      "docx:document:write_only",
      "docx:document.block:convert"
    ],
    "user": []
  }
}
```

### Step 4：重新发布应用

权限变更后必须重新发布，新权限才会生效。
进入「**版本管理与发布**」 → 创建并提交新版本，等待审核通过后上线。

### 附录：（Agent 专用）一键配置链接

> **本小节仅面向** **Agent** **执行场景**：人类用户按 Step 1–4 操作即可，跳过本小节。

如果你是 Agent 在帮用户配置本节，可以用一条一键配置 URL 让用户点击代替手动 Step 1–4 全套（事件订阅 + 长连接 + 评论权限 + 发布）。

1. 从应用配置文件中读取 App ID（形如 `cli_xxxxxxxxxxxxxxxx`）。
1. 按以下模板拼出 URL，将 `<APP_ID>` 替换为实际值：
     ```
    https://open.larksuite.com/page/launcher?clientID=<APP_ID>&tp=ccm
     ```
   - `clientID`：上一步拿到的 App ID。
   - `tp=ccm`：配置模板类型，这里传入 tp=ccm（云文档场景）。
1. 把 URL 用代码块原样发给用户（不要做 Markdown 链接化、不要 URL 编码）。用户在Lark里点开链接 → 点击页面里的「**开通并授权**」按钮即可。
> **回退条件：** 用户反馈点开 404、模板选项明显不对，或者部署环境只能用公网 `open.feishu.cn` 时，跳过本附录、按 Step 1–4 手动操作。


## 三、Agent 配置

完成第一节后，按你的部署方式继续配置 Agent。

### Agent 执行须知

> **本小节仅面向 Agent 执行场景**：人类用户可直接跳到对应选项的配置步骤。

**执行任何配置命令前，先按下表识别部署类型，向用户汇报判断结果与执行计划，等用户确认后再继续。** 按顺序检测，遇到第一个匹配即停止：
| 步骤 | 检测命令                           | 匹配条件       | 对应选项              |
| -- | ------------------------------ | ---------- | ----------------- |
| 1  | `openclaw --version`           | 命令存在且有版本输出 | 选项 A：本地 OpenClaw  |
| 2  | `ls ~/.hermes/.env`            | 文件存在       | 选项 B：Hermes Agent |
| 3  | 以上均不匹配，或用户确认使用自建框架             | —          | 选项 C：自建 Bot       |

识别完成后，向用户说明：
> "我判断你的环境是 **[选项 X：名称]** ，接下来准备执行：[步骤列表]。请确认是否继续。"
> 
确认后再进入对应选项的配置步骤。

### 选项 A：本地 OpenClaw

#### 版本要求

推荐版本：**OpenClaw 2026.4.12**

```
// 安装到 4.12 版本
npm install -g openclaw@2026.4.12

// 检查版本是否准确
openclaw --version
```

预期输出：版本号包含 `2026.4.12`。

#### 重启 OpenClaw

```
openclaw gateway restart
```
> 重启后 OpenClaw 需要初始化Lark文档相关配置，**建议等待 2–3 分钟后再发送测试评论**。

#### 验证连接状态

```
openclaw gateway status
```
预期输出：包含连接成功信息（如 `Connected to Lark` 或类似字样）。

#### （可选）开启 Full 工具模式

如果遇到 bot 无法读取文档内容或写入评论，执行：
```
openclaw config set tools.profile "full"
openclaw gateway restart
```


### 选项 B：Hermes Agent

#### 前置条件

已完成 Hermes Lark Channel 接入配置。
> 💡 **Channel 是什么：** 相当于 Hermes 里的"Lark机器人前台"，负责处理消息收发、去重、多媒体文件、卡片交互等底层逻辑，开发者只需关注用户输入、机器人回复和后续业务处理。

如未完成 Channel 接入，先参考[官方文档](https://hermes-agent.nousresearch.com/docs/user-guide/messaging/feishu)。

#### 确认 .env 配置

打开 `~/.hermes/.env`，确认以下字段存在且正确：
```
Lark_APP_ID=cli_xxx                 # Lark开放平台的 App ID

Lark_APP_SECRET=your_secret         # 对应的 App Secret

Lark_DOMAIN=feishu
Lark_CONNECTION_MODE=websocket      # 必须是 websocket（长连接）

```

#### 配置允许 @bot 的用户白名单

Hermes 默认只回复白名单内的用户，其他人 @bot 会被忽略。编辑 `~/.hermes/feishu_comment_rules.json`，在 `allow_from` 中填入允许的 open_id：

```
{
  "enabled": true,
  "policy": "allowlist",
  "allow_from": ["ou_xxxxxxxx", "ou_yyyyyyyy"],
  "documents": {}
}
```

#### 重启 Hermes

```
hermes gateway restart
```
> ⏱️ 重启后 Hermes 需要初始化Lark文档相关配置，**建议等待 2–3 分钟后再发送测试评论**。

#### 验证某用户是否已放行

```
python -m gateway.platforms.feishu_comment_rules check <fileType:fileToken> <user_open_id>
```

#### 验证连接状态

```
cat ~/.hermes/gateway_state.json
```
预期输出 `feishu.state` 为 `connected`：
```
{
  "gateway_state": "running",
  "platforms": {
    "Lark": {
      "state": "connected"
    }
  }
}
```

若 `feishu.state` 不是 `connected`，重新执行 `hermes gateway restart`。


### 选项 C：自建 Bot（其他框架）

开放平台侧配置和其他选项完全一样（按第二节做）。自建 vs A/B/C 的核心差异：选项 A/B 的 OpenClaw / Hermes 本身、以及选项 C 的官方 OpenClaw 插件，都内置了一层「长连接接收Lark事件 + dispatch」的 **gateway**——自建 bot 没有，必须自己实现。

#### 核心能力清单

1. **Gateway 层（自建必须自己写）** ：建立Lark WebSocket 长连接、接收事件、回 ack、断线重连。Lark不支持 HTTP webhook（见第二节 Step 2），长连接是唯一通路。
1. **事件 dispatch**：从 `drive.notice.comment_add_v1` 事件 payload 取 `file_token` / `comment_id` / `mention_list`，确认 bot 自己在 mention 列表里，再触发业务逻辑。
1. **调用Lark API**：推荐用 lark-cli（已封装 `tenant_access_token` 管理和请求格式）；裸 HTTP 也能跑但更费力。

#### 安装 lark-cli

```
npm install -g @larksuite/cli
npx skills add larksuite/cli -y -g
```
> 选项 A / B 已内置 lark-cli，本步骤仅适用于自建场景。

#### 回复评论示例

```
lark-cli api POST /open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies \
  --data '{"content": {"elements": [{"type": "text_run", "text_run": {"text": "这里是 bot 的回复内容"}}]}}'
```

## 四、文档授权（独立步骤）

事件和权限配置完成后，bot 还需要对具体文档有访问权限：

1. 在Lark云文档中，**hover 到评论里的 bot mention**
1. 弹出的 popover 中查看授权状态
1. 若显示「未授权」，点击授权入口完成授权

> **注意：** 文档授权是**逐文档**操作，每个需要 bot 参与的文档都需要单独授权。


## 五、验证

完成前四节的配置后，按以下步骤验证：
1. 打开任意Lark云文档（已授权给该 bot）
1. 在评论区输入 `@[你的 bot 名称]` 并发送
1. 预期结果：**bot 在 2 分钟内在该评论下回复**


## 六、排查清单

bot 未回复时，按顺序逐项检查：
| # | 检查项        | 验证方式      | 预期结果      |
| -- | ---------- | ----------------- | ------------- |
| 1  | 事件已订阅      | Lark开放平台 → 事件与回调         | 列表中有 `drive.notice.comment_add_v1` |
| 2  | 使用长连接      | Lark开放平台 → 事件配置 / 回调配置   | 均显示「长连接」                           |
| 3  | 权限齐全       | Lark开放平台 → 权限管理          | 表格中的 10 个 scope 均已申请并通过            |
| 4  | 应用已发布      | Lark开放平台 → 版本管理          | 最新版本状态为「已上线」                       |
| 5  | Agent 正在运行 | 见第三节各选项验证步骤            | 网关状态：connected                     |
| 6  | 文档已授权      | 云文档中 hover bot mention | popover 显示已授权   
|
**逐项检查完仍未解决：**
- **本地 OpenClaw**：执行 `openclaw gateway restart`，等 3 分钟后重新测试
- **Hermes**：检查 `~/.hermes/gateway_state.json` 确认 `feishu.state = connected`
- **云端 OpenClaw**：在控制台重启实例后重新测试


## 七、已知限制

- **文档授权逐文档生效**：开放平台权限配置完成后，仍需对每个文档单独授权给 bot
- **回复有延迟**：从用户发送评论到 bot 回复通常需要数秒至数十秒（Agent 处理时间）
- **Base 多维表格评论**：暂不支持
- **事件开通是必要条件，但不是充分条件**：完成本手册所有配置后，bot 能否正确回复还取决于 Agent 自身的逻辑实现
