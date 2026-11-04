---
document_id: '7704258746084609475'
directory_id: '7680851565851282871'
title: 智能体入会使用指南
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user-guide/agent-meeting-user-guide
breadcrumb:
- Server API
- Video Conferencing
- bot
- User Guide
- Meeting Bot Integration Guide
document_type: GuideDocumentType
updated_at: 2026-11-04T10:18:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user-guide/agent-meeting-user-guide
---

# 智能体入会使用指南

通过智能体入会能力，应用可以使用用户身份获取当前会议的会中内容，也可以让应用 Bot 作为独立参会者加入会议并接收实时事件。

本文介绍用户身份接入和应用身份接入两种方式，帮助开发者根据 Bot 是否需要作为独立参会者加入会议选择合适的接入路径。

## 一、能力概述

### 1.1 选择接入方式

| 业务需求 | 接入方式 | 会议中的表现 | 获取事件的方式 |
| --- | --- | --- | --- |
| 用户本人参加会议，Agent 在后台获取字幕、聊天等内容 | 用户身份接入（UAT） | 不显示 Bot 参会者 | 调用接口轮询获取 |
| Bot 作为独立参会者加入会议并接收会中动态 | 应用身份接入（TAT） | Bot 显示在参会人列表 | 事件订阅为主，接口补拉为辅 |

### 1.2 能力边界

- 用户身份接入要求授权用户本人正在目标会议中，不会创建独立的 Bot 参会者。
- 应用身份接入支持 Bot 主动入会和离会。Bot 入会后会显示在参会人列表中。
- 两种方式均可获取当前可见的会中事件，但鉴权身份和事件获取方式不同。

## 二、接入前准备

### 2.1 创建应用并开启所需能力

进入[Lark开放平台](https://open.larksuite.com/)，创建企业自建应用并获取 `App ID` 和 `App Secret`。已有企业自建应用可以复用。

如果使用应用身份接入，还需要在[应用管理后台](https://open.larksuite.com/app)进入 **功能 → 机器人**，开启机器人能力。

### 2.2 确认版本要求

- Lark客户端版本需为 **7.68 及以上**。可在Lark客户端中进入 **头像 → 关于Lark → 检查更新** 查看并升级版本。
- 如使用 lark-cli 调试，版本需为 **1.0.70 及以上**。可执行以下命令升级：

```bash
npm install -g @larksuite/cli@latest
```

如果当前客户端无法升级到 7.68 及以上版本，则暂不满足智能体入会的客户端条件。

### 2.3 开启当前会议的「允许智能体加入会议」

该设置按会议生效，需要由当前会议的 **主持人** 在会议中操作：

1. 发起或进入一场多人会议。
2. 在当前会议中开启 **AI 总结**。
3. 点击当前会议的 **安全按钮**。
4. 确认 **允许智能体加入会议** 已开启。

如果安全中未显示该选项，请先确认当前会议已经开启 AI 总结。无需前往Lark客户端的全局设置页面确认。

![当前会议中开启 AI 总结和允许智能体加入会议](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eb1ea70801866df10b82ce7ad9576824_1Nt6JC2dI5.png?lazyload=true&width=2192&height=1550)

## 三、用户身份接入（UAT）

用户身份接入适用于“用户本人在会议中，Agent 在后台读取会中内容”的场景。该方式不会在会议中创建或显示 Bot 参会者。

### 3.1 申请权限并完成用户授权

为应用申请以下用户身份权限：

```text
vc:meeting.meetingevent:read
```

在[应用管理后台](https://open.larksuite.com/app)按以下路径申请权限：

1. 进入 **开发配置 → 权限管理**。
2. 点击 **开通权限**。

![在权限管理中点击开通权限](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/843b17aa5702083fc2008a6d362d6ecf_sWNxSdWAWe.png?lazyload=true&width=3290&height=766)

3. 搜索 **获取会议信息**，切换到 **用户身份权限**，选择 `vc:meeting.meetingevent:read`。

![选择获取会议信息用户身份权限](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/31d7c20f3ffb625e936d4dee97ed7401_ggTYOiDVgp.png?lazyload=true&width=1776&height=640)

权限申请完成后发布应用版本，并引导用户完成 OAuth 授权。

使用 lark-cli 时，可以执行：

```bash
# 生成授权链接
lark-cli auth login \
  --scope "vc:meeting.meetingevent:read" \
  --no-wait \
  --json

# 用户完成授权后确认授权结果
lark-cli auth login --device-code <device_code>
```

### 3.2 获取用户当前所在的会议

授权用户进入会议后，调用[`GetUserActiveMeeting`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user_active_meeting)获取该用户当前所在的会议。

```bash
lark-cli vc +meeting-list-active --as user --format json
```

接口可能返回一个或多个会议。存在多个会议时，应根据会议主题或 9 位会议号选择目标会议。

返回结果中需要区分：

- `meeting_no`：9 位会议号，用于识别会议或调用 Bot 入会接口。
- `meeting_id`：长会议 ID，用于获取会中事件和执行后续会中操作。

### 3.3 获取会中事件

使用上一步返回的长 `meeting_id` 调用[`ListMeetingEvents`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/events)：

```bash
lark-cli vc +meeting-events \
  --as user \
  --meeting-id <meeting_id> \
  --page-all \
  --format json
```

首次调用获取当前可见的事件；持续获取时保存响应中的 `page_token`，并使用该游标继续获取新增事件。

> `meeting_id` 必须使用接口返回的长会议 ID，不要传入 9 位会议号。

## 四、应用身份接入（TAT）

应用身份接入适用于“Bot 需要作为独立参会者加入会议，并接收实时会中事件”的场景。

### 4.1 申请应用身份权限

申请以下应用身份权限：

```text
vc:meeting.bot.join:write
```

在[应用管理后台](https://open.larksuite.com/app)进入 **开发配置 → 权限管理 → 开通权限**，搜索 **加入、离开会议**，切换到 **应用身份权限**，选择 `vc:meeting.bot.join:write`。

![选择加入、离开会议应用身份权限](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc41dd1331baba2de89a56115748d91e_4jIptVfzZk.png?lazyload=true&width=1740&height=646)

如果控制台提示需要配置应用身份权限可访问的数据范围，请根据业务需要选择 **全部** 或 **按条件筛选**。下图仅展示一种按条件筛选的配置示例，实际范围以应用的可用范围和业务需求为准。

![应用身份权限可访问的数据范围配置示例](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/13bb4d8bb1b1524d03f3e7d6c8527bd7_Z6afznpE1v.png?lazyload=true&width=1758&height=1012)

权限变更后，需要创建并发布新版本，确保权限配置已经生效。

### 4.2 配置事件订阅

在[应用管理后台](https://open.larksuite.com/app)按以下路径配置事件订阅：

1. 进入 **开发配置 → 事件与回调 → 事件配置**。
2. 点击 **添加事件**。

![在事件配置中点击添加事件](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/529567ae470e1f9e3ef940fdfcd9d5f6_9XCgzY4MP9.png?lazyload=true&width=3338&height=892)

3. 搜索 `vc.bot`，切换到 **应用身份订阅**，选择以下事件。

![搜索并选择智能体入会相关事件](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b5fec07b65880f151efb4428f3457a59_3jHSkm0M0D.png?lazyload=true&width=1652&height=786)

| 事件 | 用途 |
| --- | --- |
| `vc.bot.meeting_invited_v1` | Bot 在会议中被邀请时触发 |
| `vc.bot.meeting_ended_v1` | 会议结束时触发 |
| `vc.bot.meeting_activity_v1` | 推送字幕、聊天、参会人变化和共享文档等会中活动 |

事件名称必须与开放平台提供的事件定义完全一致。

### 4.3 触发 Bot 入会

Bot 可以通过以下方式获取目标会议：

- 接入方获得 9 位会议号后，主动调用[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)接口。
- 用户在会议中邀请 Bot，接入方收到 `vc.bot.meeting_invited_v1` 事件后调用[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)接口。

收到邀请事件不代表 Bot 已经加入会议。接入方仍需从事件中取得会议信息并调用[`BotJoinMeeting`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)；只有接口调用成功后，Bot 才会成为会议参会者。

```text
用户在会议中邀请 Bot
        ↓
接入方收到 vc.bot.meeting_invited_v1
        ↓
解析会议号并调用 BotJoinMeeting
        ↓
Bot 加入会议并开始接收会中事件
```

### 4.4 调用入会接口

使用应用身份调用[`BotJoinMeeting`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)：

```bash
lark-cli vc +meeting-join \
  --as bot \
  --meeting-number 123456789 \
  --format json
```

如果会议设置了密码，还需要传入正确的会议密码。入会成功后保存响应中的长 `meeting_id`，后续获取事件和主动离会均使用该 ID。

### 4.5 接收事件并补拉历史事件

Bot 在会议中时，通过 `vc.bot.meeting_activity_v1` 接收实时事件。

如果事件连接中断，可以使用长 `meeting_id` 调用[`ListMeetingEvents`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/events)补拉当前可见的历史事件：

```bash
lark-cli vc +meeting-events \
  --as bot \
  --meeting-id <meeting_id> \
  --page-all \
  --format json
```

推送链路和补拉链路可能包含同一业务事件，接入方应基于事件 ID 以及各类型数据的唯一标识进行幂等处理。

### 4.6 离开会议

Bot 可以主动调用[离开会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave)接口：

```bash
lark-cli vc +meeting-leave \
  --as bot \
  --meeting-id <meeting_id> \
  --format json
```

会议自然结束时，接入方会收到 `vc.bot.meeting_ended_v1`。主动离会或会议结束后，应清理该会议对应的游标、连接和临时状态。

## 五、查看会中事件的数据结构

用户身份通过[`ListMeetingEvents`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/events)拉取会中事件，应用身份通过 `vc.bot.meeting_activity_v1` 接收会中事件。有关事件类型、字段说明、数据结构和示例，请参见[会中事件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/supportEventsList/meeting_activity)。

## 六、常见问题与解决方案

### 6.1 会议中无法搜索或邀请 Bot

依次确认：

1. 应用已开启机器人能力。
2. 包含最新权限配置的应用版本已发布并生效。
3. 当前用户在应用的可用范围内。
4. 当前会议已开启 AI 总结和「允许智能体加入会议」。
5. Lark客户端版本为 7.68 及以上。

### 6.2 收到邀请事件后 Bot 未加入会议

`vc.bot.meeting_invited_v1` 仅表示 Bot 被邀请，不表示 Bot 已加入会议。接入方需要在收到事件后调用[`BotJoinMeeting`](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)，并检查接口返回结果。

### 6.3 接口提示会议未允许智能体接入

如果接口返回以下提示：

> 该智能体暂时无法加入会议。请由会议主持人开启「AI 总结」后重试；如已开启，请在当前会议的「安全」中确认「允许智能体加入会议」已开启。

请由当前会议的 **主持人** 在会议中依次确认：

1. 当前会议已开启 AI 总结。
2. 当前会议的 **安全 → 允许智能体加入会议** 已开启。

该设置针对当前会议生效，无需前往Lark客户端的全局设置页面确认。

### 6.4 入会或离会接口提示 Token 类型不支持

`BotJoinMeeting` 和 `BotLeaveMeeting` 使用应用身份鉴权。请确认请求使用 Tenant Access Token，不要使用 User Access Token 调用 Bot 入会或离会接口。

### 6.5 `meeting_id` 格式不正确

- 调用 Bot 入会接口时使用 9 位 `meeting_no`。
- 入会成功后保存响应中的长 `meeting_id`。
- 获取会中事件和主动离会时使用长 `meeting_id`。

### 6.6 已入会但未收到会中事件

依次确认：

1. Bot 或授权用户仍在目标会议中。
2. 鉴权身份与 `meeting_id` 的来源身份一致。
3. 应用已订阅正确的事件，并已发布包含该配置的版本。
4. 程序按照 `activity_event_type` 读取了对应的 items 数组。
5. 去重和游标逻辑没有错误地丢弃新增事件。

### 6.7 用户身份接入时会议中不显示 Bot

这是用户身份接入的正常表现。用户身份接入不会创建独立的 Bot 参会者。如果业务要求 Bot 显示在参会人列表中，请改用应用身份接入。

## 七、附录：权限、事件与鉴权方式

### 7.1 权限

| 权限 | 身份类型 | 适用场景 |
| --- | --- | --- |
| `vc:meeting.meetingevent:read` | 用户身份 | 查询用户当前会议并获取会中事件 |
| `vc:meeting.bot.join:write` | 应用身份 | Bot 加入和离开会议 |

### 7.2 事件

| 事件 | 适用场景 |
| --- | --- |
| `vc.bot.meeting_invited_v1` | Bot 被邀请加入会议 |
| `vc.bot.meeting_ended_v1` | Bot 所在会议结束 |
| `vc.bot.meeting_activity_v1` | Bot 接收会中活动 |

### 7.3 鉴权方式

| 操作 | 鉴权方式 |
| --- | --- |
| 查询授权用户当前会议 | User Access Token |
| 用户身份获取会中事件 | User Access Token |
| Bot 加入或离开会议 | Tenant Access Token |
| Bot 获取会中事件 | Tenant Access Token |
