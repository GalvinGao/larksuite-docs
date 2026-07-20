---
document_id: '7275897728244613126'
directory_id: '7273792780344868870'
title: 简介
full_path: /home/quickly-develop-interactive-cards/introduction
breadcrumb:
- Developer Guides
- Quick Starts
- Develop Interactive Cards
- Introduction
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:57Z
source_url: https://open.larksuite.com/document/home/quickly-develop-interactive-cards/introduction
---

# 简介

本文介绍如何利用消息卡片进行消息回复、处理用户在卡片上的操作，以及如何更新已发送的消息卡片。通过本教程，你可以了解如何快速构建一个简单的交互式消息卡片，并了解消息卡片支持的事件回调处理机制。为了方便演示，本教程以一个模拟审批机器人进行交互。
- 提供一个审批机器人，用文字及消息卡片回复用户指令消息。
- 响应用户在卡片上进行的请假、加班操作，并更新消息卡片。

## 什么是消息卡片

消息卡片是一种以标题、描述、图片和按钮等元素组成的消息类型，可在Lark客户端中以卡片形式展示，它可以使消息接收者更直观地了解和处理发送的信息。消息卡片可以承载如审批、投票统计、报警处理等互动操作，也可以用于呈现多种类型的静态内容，如醒目的通知、图文并茂的文章列表等。更多内容参考 [消息卡片概述](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)。

## 操作流程

本文涉及的操作流程如下图所示：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ef4a718ec641b69a5a38e2d4e39b174c_sMTwqmB7am.png?height=208&lazyload=true&width=853)

## 使用到的 OpenAPI 列表
:::html
<md-table>

<md-thead>

<tr>

<md-th style="width: 50%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 30%;">权限要求（满足任一）</md-th>

<md-th style="width: 20%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" > [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)  `POST`  /open-apis/im/v1/messages
  </md-text>

</md-td>
  
<md-td>
  <md-perm name="im:message" desc="获取与发送单聊、群组消息" tags="">获取与发送单聊、群组消息</md-perm>
  <md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" tags="">以应用的身份发消息</md-perm>
</md-td>
  
<md-td>
<md-tag type="token-tenant">tenant_access_token</md-tag>
</md-td>
</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" > [延时更新消息卡片](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)  `POST`  /open-apis/interactive/v1/card/update
  </md-text>

</md-td>
  
<md-td>
  /
</md-td>
  
<md-td>
<md-tag type="token-tenant">tenant_access_token</md-tag>
</md-td>
</md-tr>
