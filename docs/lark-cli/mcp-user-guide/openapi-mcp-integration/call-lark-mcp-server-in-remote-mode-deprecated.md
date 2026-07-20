---
document_id: '7527610399324864524'
directory_id: '7506726087805566988'
title: 远程调用 Lark MCP Server（不推荐）
full_path: /mcp_open_tools/call-feishu-mcp-server-in-remote-mode
breadcrumb:
- Lark CLI
- MCP User Guide
- OpenAPI MCP Integration
- Call Lark MCP server in remote mode (Deprecated)
document_type: GuideDocumentType
updated_at: 2026-04-24T06:05:32Z
source_url: https://open.larksuite.com/document/mcp_open_tools/call-feishu-mcp-server-in-remote-mode
---

# 远程调用 Lark MCP Server（不推荐）

Lark开放平台提供远程 MCP Server 配置功能，支持  Streamable HTTP 传输协议，满足用户在云场景中，安全高效地通过 Cursor 、Trae、Claude、 n8n 等 AI agent 一键集成并使用丰富的Lark开放能力。


## 配置 MCP Server

在Lark MCP 配置平台创建 MCP 远程服务，自定义添加工具（即Lark开放平台服务端 API），灵活构建业务所需的 MCP 工具。

:::note
该工具均以当前登录用户身份（user_access_token）调用 API。相关说明：
- 在配置功能过程中，系统会引导你一键完成登录用户的授权操作，只有在用户授权后，MCP Server 才可以以用户身份调用工具。
- user_access_token 本身存在有效期，远程 MCP Server 配置功能已支持过期前自动刷新有效期的能力。
- 了解 user_access_token 可参考 [获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/get-user-access-token)、[刷新 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user-access-token)。
:::

1. 登录[Lark MCP 配置平台](https://open.larksuite.com/page/mcp)。

	:::warning
    远程 MCP Server 配置功能正在内测中，如无法访问配置平台，可联系客户成功经理开通内测。
    :::
    
3. 在页面左侧，点击 **创建 MCP 服务**。
4. 在服务创建页面，完成以下配置。

	1. 在 **MCP 工具配置** 区域，确认当前用户身份。
	  
		后续使用 MCP 管理Lark业务资源时，均是以当前显示的用户进行的，因此，你需要确认当前用户身份正确，若不正确则需要退出登录，并使用正确的用户重新登录。
         
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/91a132785a1240756a97a43b4767cab5_Dc1LKQLHnI.png?height=592&lazyload=true&maxWidth=600&width=1882)
        
	2. 在 **添加工具** 卡片内，点击 **添加**。
	3. 在 **选择工具** 对话框，选择所需的工具。

		平台提供了 **常用场景** 供你选择，若常用场景没有满足需求的工具，可以在 **自定义选择** 列表中灵活选择多种工具。例如，你的业务需要 MCP 具有管理Lark多维表格的能力，则选中多维表格。  
        
        :::note
        展开工具列表，点击某一工具右侧的 **查看文档**，可查看工具的详细介绍。
        :::

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fdbe0215345764f90c26668b41bd06dd_1lhE2UMG9Y.png?height=1414&lazyload=true&maxWidth=600&width=1644)

	4. 点击 **确认添加**，并在弹出的 **获取用户授权** 对话框，确认授权用户登录信息、Lark MCP 应用获得的权限信息后，点击 **授权**。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f47321bd62335da9460f14a0b74818e2_jVgA41QT6g.png?height=1322&lazyload=true&maxWidth=400&width=1238)
         
4. 成功添加后，在 **如何使用 MCP 服务** 区域，选择 **传输方式**，并查看 **服务器 URL、JSON**。

	:::note
    - 其中包含的链接代表以当前登录用户身份去调用Lark工具，相当于个人密钥，请勿泄露给其他人。
	- 链接存在有效期，过期自动失效，如需延长有效期，可以在 URL/JSON 下方点击 **重新授权**。
	- 如担心链接已经泄露，可以在 URL/JSON 下方点击 **重置链接**，将原链接失效并生成新的链接。
    :::

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/efdf2ef6eba1050c8fd4c84c9c5b3d43_PJSMRla9d2.png?height=626&lazyload=true&maxWidth=600&width=1606)
    
	- **传输方式**：保持默认 Streamable HTTP 即可。Streamable HTTP 是基于 HTTP 协议的分块（chunked）流式传输，支持任意格式数据（如文件、日志）的渐进式传输。
	- **服务器 URL、JSON**：根据需要将链接配置到 AI agent 内，使 AI agent 能够远程连接Lark MCP 服务。
    
    	Trae/Cursor 支持一键安装，操作说明参考本文下一章节。其他 AI agent 可以在相应的 MCP 配置界面设置 URL 或 JSON 来连接Lark MCP 服务。
        
## 一键安装到 Trae/Cursor

配置Lark MCP 服务后，在服务页面底部已提供了快捷添加到 Trae、Cursor 的入口，供你快速安装到 Trae、Cursor。

:::note
该方式需要确保本地已安装 [Trae](https://trae.ai/) 或者 [Cursor](https://cursor.com/)。
:::

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e601b2a82d0aec056dccc36ae1a9cb66_KMDawJ0eXw.png?height=676&lazyload=true&maxWidth=600&width=1010)

以 Cursor 为例，操作如下：

1. 点击 **快捷添加到 Cursor**，并根据指引打开 Cursor 客户端。
2. 在 **MCP Tools** 区域的 **Install MCP Server？** 卡片内，点击 **Install**。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/931c35ee2aa25f3c65999e9bad4adc83_WqxR6EU5Ay.png?height=500&lazyload=true&maxWidth=400&width=1370)
    
  	成功添加后 MCP Tools 列表显示如下图所示。
  
  	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/31e063790d4a3ee1a0fb8a76a6b23cc9_9NJpV2MPy0.png?height=362&lazyload=true&maxWidth=400&width=1324)

## 使用 MCP

以 Cursor 为例，在 AI agent 内使用Lark MCP 服务。

1.  在工具右侧打开一个新会话（New Chat）。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2cd3a244e1f2b1fc647e9144e9db8f1b_11E3PELFS5.png?height=340&lazyload=true&maxWidth=400&width=884)
    
    - 左下角选择 **Agent**。
    - 大模型选择 **Auto** 或者手动选择任一模型（不同模型实现效果存在差异）。

2. 将具体的业务需求填写至输入框。

    本章节提供以下示例场景供你参考。
    
    1. 编写提示词，让智能体根据一个多维表格的截图示例，用Lark开放接口实现一个批量读取云文档的内容并将信息提取写入到多维表格的需求，最后通过Lark云文档撰写技术方案。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a36b12f780bf036e8d513a66644a9500_8GhXTeYymk.png?height=271&lazyload=true&maxWidth=400&width=817)

		智能体返回技术方案云文档链接：
        
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/36f38346b0e53b974e732492c5af34ba_pnh7CyfOQM.png?height=287&lazyload=true&maxWidth=400&width=809)

        
        访问链接查看文档内容。
               
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/812636a46ad087938a5ea19d482f8ac8_aC3xxA5eq5.png?height=612&lazyload=true&maxWidth=400&width=802)
        
        
	2. 继续编写提示词，让智能体批量生成十篇Lark云文档，填充模拟数据，然后根据以上技术方案生成实现代码，并确保测试通过。

		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/08b5330f79cab457f480a709efb38b12_wgLA5rqZ2B.png?height=420&lazyload=true&maxWidth=400&width=1168)
        
        智能体会先批量生成十篇云文档。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6f9327833222626d7da60fbea8711cc9_RrI4PC30Yb.png?height=1028&lazyload=true&maxWidth=600&width=2360)
        
        文档内容示例：
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/35bac41762f994f1d67eb6da4b76a4a6_6fYdERr5he.png?height=796&lazyload=true&maxWidth=400&width=1304)
        
        然后智能体会输出对应技术方案的项目代码。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0044e515825dd606c0cd21bc66b985b0_ryzB5ufiJY.png?height=1364&lazyload=true&maxWidth=400&width=1158)
        
	3.  根据智能体返回的测试数据要求，发送应用的 App ID、App Secret，以及多维表格的 AppToken、TableId。

		:::note
        需前往创建企业自建应用和多维表格资源，以用于测试。相关操作参见：
        - [企业自建应用开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process)
        - [如何获取应用的 App ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)
        - [多维表格概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/bitable-overview)
        :::

		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f933a524c84a72f99fe62412b9b943ae_Z9cGUg3BhS.png?height=480&lazyload=true&maxWidth=400&width=1170)
        
        等待智能体测试完成，并得到返回结果。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/016f3e3ec0544bcb6349b972a33b744a_htvxQ9VSBV.png?height=175&lazyload=true&maxWidth=400&width=615)


	4. 查看对应的多维表格，确认测试结果符合预期。
	
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/29cb608eec39499fa71ee3bceba1759c_1qDTIQs7Qr.png?height=448&lazyload=true&maxWidth=600&width=905)
        
        
        
    5. 在 AI IDE 内应用已通过测试的代码。
    
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/41514979415c6497cec1a4c602cca2f9_xXwNpxUZ35.png?height=700&lazyload=true&maxWidth=600&width=1611)





## 常见问题

### 如何重命名或删除已配置的 MCP Server？

1. 登录[Lark MCP 配置平台](https://open.larksuite.com/page/mcp)。
2. 在左侧 **已创建的 MCP 服务** 列表，选择需要操作的服务。
3. 在标题区域，点击 **···**，可选择 **重命名** 或 **删除** 当前服务。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2f650c1e5132e40facb53c328bc088bb_Ykr5xbwa6s.png?height=288&lazyload=true&maxWidth=600&width=1646)
    
### 配置工具后，如何修改？修改后是否需要重新生成服务器 URL？

1. 登录[Lark MCP 配置平台](https://open.larksuite.com/page/mcp)。
2. 在左侧 **已创建的 MCP 服务** 列表，选择需要操作的服务。
3. 在 **MCP 工具配置** 区域的 **添加工具** 卡片内，点击 **编辑**，修改已选择的工具，并保存即可。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fe4db918638d73a1004f7bc539d0b7a2_qAz1q544k0.png?height=572&lazyload=true&maxWidth=600&width=1628)
    
4. 修改后，需要在 AI 工具内刷新或重启 MCP 服务。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/55d43f50d294937c09386a672668b605_sr1ltKM3qt.png?height=464&lazyload=true&maxWidth=400&width=1094)
    
### Cursor 内的 MCP Tools 状态显示异常是什么原因？

- 问题现象：如下图，将 MCP 服务一键安装到 Cursor 后，工具状态指示灯显示为红色。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fcfd3244cc2650e81f27d13f7a57fb9a_ZYDXFRqKNf.png?height=260&lazyload=true&maxWidth=400&width=1638)
- 问题原因：受工具影响，状态指示灯可能显示为红色，一般不影响 MCP 工具的使用。
- 解决方案：可忽略该状态，直接在 AI Chat 中发送需求，根据 AI 响应结果判断是否可以正常调用Lark MCP 工具，如果可以即没有问题。如果无法调用，请重新配置远程 MCP 服务并一键安装到 Cursor 后重试。
