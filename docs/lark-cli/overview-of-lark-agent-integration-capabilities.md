---
document_id: '7642539595384671670'
directory_id: '7506726087805534220'
title: Lark Agent 集成能力概述
full_path: /mcp_open_tools/overview-of-lark-agent-integration-capabilities
breadcrumb:
- Lark CLI
- Overview of Lark Agent Integration Capabilities
document_type: GuideDocumentType
updated_at: 2026-05-25T06:59:49Z
source_url: https://open.larksuite.com/document/mcp_open_tools/overview-of-lark-agent-integration-capabilities
---

# Lark Agent 集成能力概述

Lark面向 Agent 开发者提供三类可组合能力，它们分别解决三个不同问题：
  
  :::html
<md-table style="width: 1000px;">
  <md-tbody>
     <md-tr>
      <md-td>**[一键创建Lark应用](/document/mcp_open_tools/integrating-agents-with-feishu/overview)**：为你的用户快速创建一个Lark智能体应用，并预置常见权限与事件订阅。![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ee1a955c197872a4d739a1a0a2c2f137_5i4OTqIk5z.png?height=840&lazyload=true&maxWidth=310&width=988)</md-td>
      <md-td>**[Channel SDK](/document/mcp_open_tools/integrating-agents-with-feishu/integrate-feishu-channel)**：让 Agent 进入Lark会话，在群聊、单聊和文档评论里稳定收发消息，与用户实时对话。![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/01fe9eefe325fb8ebcfd155a4651df4f_PY1XMqe7XS.png?height=1250&lazyload=true&maxWidth=350&width=2338)</md-td>
      <md-td>**[Lark CLI](/document/mcp_open_tools/feishu-cli-let-ai-actually-do-your-work-in-feishu)**：让 Agent 直接操作Lark里的业务对象，例如文档、日历、表格、邮件和任务。![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7e3b61ae1387b2212597ea3bcb28f0a8_c2ZLPcGhf5.png?height=966&lazyload=true&maxWidth=350&width=1304)</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::


## 能力一览

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d367f595768e1cd2053b844c88d8e7a7_VNgPpQji4z.png?lazyload=true&width=2720&height=1608)

| 能力       | 什么时候使用    | 形态    | 接入指引    |
| ------------ | -------------- | ------ | --------- |
| 一键创建Lark应用 <br><br> **==凭据层==**    | 快速获取应用凭据（`App ID`、`App Secret`），并自动完成常见权限与事件订阅的初始配置，把“连接Lark”变成一次扫码动作。<br><br> **目标**：**快速获得可用Lark凭据** | Web 扫码 SDK <br><br>支持 Node.js、Python、Java、Go          | [一键创建Lark应用](/document/mcp_open_tools/integrating-agents-with-feishu/overview)                    |
| Channel SDK <br><br> **==交互层==** | 让 Agent 在群聊、单聊、文档评论中收发消息，支持流式回复与卡片交互。无需自己处理连接、事件解析和回复渲染。<br><br>**目标**：**让 Agent 能「听」和「说」**       | 通道 SDK<br><br> 支持 Node.js、Python、Java、Go              | [Channel SDK](/document/mcp_open_tools/integrating-agents-with-feishu/integrate-feishu-channel) |
| Lark CLI <br><br> **==执行层==**      | 让 Agent 真正执行Lark里的业务动作，把文档、日历、表格、邮件、任务等能力变成 Agent 可直接调用的工具。<br><br>**目标**：**让 Agent 能直接「干活」**       | CLI<br><br> 通过 `npx @larksuite/cli@latest install` 安装 | [Lark CLI](/document/mcp_open_tools/feishu-cli-let-ai-actually-do-your-work-in-feishu)
