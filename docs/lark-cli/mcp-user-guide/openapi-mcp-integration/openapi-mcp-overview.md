---
document_id: '7507202826176348172'
directory_id: '7506726087805566988'
title: OpenAPI MCP 概述
full_path: /uAjLw4CM/ukTMukTMukTM/mcp_integration/mcp_introduction
breadcrumb:
- Lark CLI
- MCP User Guide
- OpenAPI MCP Integration
- OpenAPI MCP overview
document_type: GuideDocumentType
updated_at: 2025-07-01T02:35:32Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/mcp_integration/mcp_introduction
---

# OpenAPI MCP 概述

OpenAPI MCP 工具（后续简称“MCP 工具”）可以帮助你快速实现 AI agent 与Lark开放能力的集成，实现基于 agent 的Lark云文档处理、会话管理、日历安排等自动化场景。

## 功能特性

MCP 工具使用 MCP（Model Context Protocol） 协议连接Lark开放平台，提供丰富的Lark OpenAPI 工具集，可覆盖Lark的核心功能（如消息、群组、日历、多维表格等），同时支持应用访问凭证（tenant_access_token）和用户访问凭证（user_access_token），确保 API 的调用安全可控。MCP 工具特点如下：

- 提供简单易用的命令行接口，实现快速配置与启动。
- 支持多种配置方式，适应不同的使用场景。
- 与主流 AI 工具（如 Trae、Cursor、Claude）无缝集成。
- 配置应用信息后，可自动获取应用访问凭证；登录 MCP 并完成用户登录鉴权后，可自动获取用户访问凭证，并支持自动刷新。

## 典型案例

:::note
AI 会根据实际输入的需求提示词进行规划，并调用不同的 OpenAPI，因此应用需要开通的权限也会存在不同。
:::

:::html

<md-table>
<md-thead>
<md-tr>
<md-th style="width:35%">场景示例</md-th>
<md-th style="width:35%">前置配置</md-th>
<md-th style="width:25%">Trae 运行示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>初始化项目管理多维表格
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/afc6b88dd1fc06f8b475a05f975a1074_dNfEjWXykn.png?height=786&lazyload=true&width=2936)
  
  </md-td>
<md-td>-   **应用能力**：机器人
- **应用权限**：

	<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm></md-td>
<md-td>
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f32497b673136605eee1f624c63d67bc_YrXsa60jjH.png" width="270" height="500">  
</md-td>

</md-tr>

<md-tr>
<md-td>创建群聊、拉人进群并发送Lark卡片
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/284ae501535ae4540ba2fdfcf6a18d70_wxX5k2uqcs.png?height=618&lazyload=true&width=2186)
  
  
  </md-td>
<md-td>-   **应用能力**：机器人
- **应用权限**：
  
  <md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm>
  <md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" support_app_types="custom,isv" tags="">以应用的身份发消息</md-perm>

- **其他**：确保群内所有成员在应用的可见范围内。参考[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。</md-td>
<md-td>
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c89088acd529a86c4549a687af780a3_WelQJWjC7v.png?height=1866&lazyload=true&width=920)
</md-td>

</md-tr>
  
<md-tr>
<md-td>总结群内的消息，并记录到多维表格
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a0d84feb93cca71a039e176a7d02ebf8_eCspZKbokF.png?height=642&lazyload=true&width=2942)
  
  
  </md-td>
<md-td>-   **应用能力**： 机器人
- **应用权限**：
  <md-perm name="im:message.group_msg" desc="获取群组中所有消息（敏感权限）" support_app_types="custom,isv" tags="">获取群组中所有消息（敏感权限）</md-perm>
  <md-perm name="base:app:create" desc="创建多维表格" support_app_types="custom,isv" tags="">创建多维表格</md-perm>
  <md-perm name="base:table:create" desc="新增数据表" support_app_types="custom,isv" tags="">新增数据表</md-perm>
  <md-perm name="base:record:create" desc="新增记录" support_app_types="custom,isv" tags="">新增记录</md-perm>

- **其他**：机器人在群聊中</md-td>
<md-td>
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c4c94c8151ae6cb0cc8185a535c9aa9d_zGuWXFV3BY.png" width="270" height="500">   
</md-td>

</md-tr>
  
<md-tr>
<md-td>为群成员开通多维表格权限
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2b39bdbfd41e273eb8280acad0517a72_O9N1tfY1qn.png?height=698&lazyload=true&maxWidth=240&width=970)
  
  </md-td>
<md-td>-   **应用能力**：启用机器人能力
- **应用权限**：
	<md-perm name="im:chat.members:read" desc="查看群成员" support_app_types="custom,isv" tags="">查看群成员</md-perm>
	<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm>

- **其他**：机器人在群聊中；机器人对多维表格有管理权限</md-td>
<md-td>
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4ff4edab9a5db801ad79857b5369dd92_F27tuhGpwU.png?height=1156&lazyload=true&width=924)</md-td>
</md-tr>

</md-tbody>
</md-table>
:::


## 支持的 OpenAPI

你可以查看 [tools](https://github.com/larksuite/lark-openapi-mcp/tree/main/docs) 获取 MCP 工具支持的Lark OpenAPI 列表，也可以通过 API 文档判断该 API 是否在 MCP 工具内支持。例如，打开一篇 API 开发文档，如 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)。若存在 **「尝试一下」** 按钮，则表示该 API 在 MCP 工具内可用。

:::warning
- 灰度中的 API（即 API 文档开头标注了 **仅部分企业可见** 的 API），在 MCP 工具内暂不支持使用。
- 图片或文件上传/下载相关的 API 在 MCP 工具内暂不支持使用。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5048ca8cf0e1f94dad9cef5f5e1d2506_kKmpwTcnp4.png?height=426&lazyload=true&maxWidth=600&width=1748)

## MCP 工具命名规范

MCP 工具按照 `biz.version.resource.method` 模式命名，与服务端 Node SDK 的方法命名一致。可按照如下方式获取 OpenAPI 在 MCP 工具内的名称。

在 API 文档中，点击 **尝试一下**，并查看 **Node SDK** 示例代码，其中 `client` 方法后的函数即 API 在 MCP 工具内的名称。例如下图所示，发送消息 API 的名称为 `im.v1.message.create`。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4a7279b0e44c6c3cd82dfac89a5d1b1d_j1HQNIB9CS.png?height=504&lazyload=true&maxWidth=600&width=891)
