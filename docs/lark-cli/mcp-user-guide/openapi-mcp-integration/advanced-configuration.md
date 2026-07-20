---
document_id: '7507202826176380940'
directory_id: '7506726087805566988'
title: 高级配置
full_path: /uAjLw4CM/ukTMukTMukTM/mcp_integration/advanced-configuration
breadcrumb:
- Lark CLI
- MCP User Guide
- OpenAPI MCP Integration
- Advanced configuration
document_type: GuideDocumentType
updated_at: 2025-07-11T04:01:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/mcp_integration/advanced-configuration
---

# 高级配置

MCP 工具支持丰富的命令参数，你可以灵活配置合适的 MCP 启动命令，各参数详细使用说明可参见下文 **参数使用示例**。

## 前提条件

已安装 lark-mcp 工具。安装方式如下：

1. 在本地终端命令行执行以下命令，安装 lark-mcp。

	```bash
    npm install @larksuiteoapi/lark-mcp -g
    ```
    
    :::note
    如果终端回显 `It is likely you do not have the permissions to access this file as the current user`，则需以管理员身份执行：`sudo npm install @larksuiteoapi/lark-mcp@beta -g`。
    :::
    
2. 执行 `lark-mcp -V` 确认 MCP 版本。

    若版本低于 0.4.0，建议卸载 lark-mcp 后重装：
	1. 卸载 lark-mcp：`npm uninstall @larksuiteoapi/lark-mcp -g`
	2. 重装 lark-mcp：`npm install @larksuiteoapi/lark-mcp -g`
	3. 再次执行 `lark-mcp -V`，确认版本信息。
    
3. （可选）全局安装 OpenAPI MCP。

    MCP 工具的安装方式分为全局安装、NPX 安装，选择任一方式安装即可使用工具。你可以根据实际需求，选择适配的方式进行安装。
    
    - 全局安装：安装在全局路径（/usr/local/lib），命令持久化，系统级别命令均可用。适用于经常需要使用命令行工具的场景。
    - NPX 安装：在[安装并使用 OpenAPI MCP](/document/uAjLw4CM/ukTMukTMukTM/mcp_integration/mcp_installation)文档中，介绍了通过 NPX 方式安装 OpenAPI MCP 的操作步骤，这种方式特点是可以快速安装上手使用，但因安装在系统临时缓存，导致命令不能持久化，运行即用即删。适用于快速试用或一次性使用的场景。

	在本地安装 lark-mcp 后，你便可以持久使用 MCP 工具（支持的命令参数参考下文 **命令参数**），以 [Cursor](https://www.cursor.com/) 为例，全局安装步骤说明如下：
    
    1. 完成准备工作。

		详情参见[安装并使用 OpenAPI MCP](/document/uAjLw4CM/ukTMukTMukTM/mcp_integration/mcp_installation#1cd67c23)。
    
    2. 打开 Cursor 工具。

	3. 打开工具内的终端。
        
    4. 执行以下命令，以用户身份登录 OpenAPI MCP。

		```bash
        lark-mcp login -a <your_app_id> -s <your_app_secret>
        ```
        
        其中 `<your_app_id>` 为Lark应用的 App ID、`<your_app_secret>` 为Lark应用的 App Secret，你可登录Lark开发者后台，在已创建的自建应用详情页的 **凭证与基础信息** 页面，获取 **App ID** 和 **App Secret**。
          

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/645c7814f5c98789f70cbd121b4f82e7_A56ZQ6Y7RL.png?height=376&lazyload=true&maxWidth=600&width=2614)      
        
    5. 终端会回显用户授权的 URL，需在 60 秒内访问该 URL 并完成授权。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/adcf99e95eb5123530695448dd590b4c_RlwPILaXO1.png?height=262&lazyload=true&maxWidth=600&width=1650)

		授权页面如下图所示，确保用户身份符合预期，并单击 **授权**，使 MCP 工具获取到用户访问凭证（user_access_token）。
        
        
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4270d66bd57fc8f95a99550968ddc6ee_f4koI2F1Oq.png?height=1106&lazyload=true&maxWidth=400&width=1220)
        
        成功授权后，终端将回显 `success`。
        
    6. 在工具的设置中，选择 **Tools & Integrations > MCP Tools > Add Costom MCP**。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0c8acc0ba1ed55004f18a65648bf53d7_49dcP1GIB8.png?height=1734&lazyload=true&maxWidth=600&width=2944)

	7. 将默认内容替换为以下 JSON，并保存。

		```json
        {
          "mcpServers": {
            "lark-mcp": {
              "command": "lark-mcp",
              "args": [
                "mcp",
                "-a",
                "<your_app_id>",
                "-s",
                "<your_app_secret>",
                "-d",
                "https://open.larksuite.com/",
                "--oauth"
              ]
            }
          }
        }
		```
        
        其中 `<your_app_id>` 为Lark应用的 App ID、`<your_app_secret>` 为Lark应用的 App Secret，你可登录Lark开发者后台，在已创建的自建应用详情页的 **凭证与基础信息** 页面，获取 **App ID** 和 **App Secret**。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/645c7814f5c98789f70cbd121b4f82e7_A56ZQ6Y7RL.png?height=376&lazyload=true&maxWidth=600&width=2614) 

	8. 配置完成后，需要关闭并重新启用 lark-mcp。

		![img_v3_02nc_38278bc8-c2f1-4933-8a60-d15e56f5d5ag.gif](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/07d8f515bc35407bed8305b4586a830c_uHPHgwsff7.gif?height=112&lazyload=true&maxWidth=600&width=1212)
    
    	正常运行的 lark-mcp 如下图所示。
    
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f0f455ddc62694984f60b4e2b0eefb14_dqhv1LvVPk.png?height=372&lazyload=true&maxWidth=600&width=591)
    
## 命令参数

### lark-mcp login

`lark-mcp login` 用于以用户身份登录并获取用户身份凭证（user_access_token），支持的命令参数说明如下表所示。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:25%">参数</md-th>
<md-th style="width:15%">简写</md-th>
<md-th style="width:30%">描述</md-th>
<md-th style="width:30%">示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>`--app-id`</md-td>
<md-td>`-a`</md-td>
<md-td>Lark应用的 App ID。</md-td>
<md-td>`-a cli_xxxx`</md-td>
</md-tr>

<md-tr>
<md-td>`--app-secret`</md-td>
<md-td>`-s`</md-td>
<md-td>Lark应用的 App Secret。</md-td>
<md-td>`-s xxxx`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--domain`</md-td>
<md-td>`-d`</md-td>
<md-td>Lark开放平台 API 域名，默认为Lark域名。</md-td>
<md-td>`-d https://open.larksuite.com`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--host`</md-td>
<md-td>无</md-td>
<md-td>监听主机，默认为 localhost。</md-td>
<md-td>`--host 0.0.0.0`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--port`</md-td>
<md-td>`-p`</md-td>
<md-td>监听端口，默认为 3000。</md-td>
<md-td>`-p 3000`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--scope`</md-td>
<md-td>无</md-td>
<md-td>用于指定用户身份凭证（user_access_token）的 API 权限。
  
- 手动指定的权限必须在应用已申请的 API 权限范围内。为应用申请权限的方式参考[申请 API 权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)。
- 如果需要实现自动刷新 user_access_token，则手动指定权限时，必须带上 `offline_access` 权限。详细介绍参考[刷新 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user-access-token)。
- 不使用 `--scope` 命令参数指定 API 权限时，默认授予用户应用所有已申请的 API 权限。
  </md-td>
<md-td>`-- scope offline_access doc:document`</md-td>
</md-tr>

</md-tbody>
</md-table>
:::



### lark-mcp logout

`lark-mcp logout` 用于登出用户，清除用户身份凭证（user_access_token），支持的命令参数说明如下表所示。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:25%">参数</md-th>
<md-th style="width:15%">简写</md-th>
<md-th style="width:30%">描述</md-th>
<md-th style="width:30%">示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>`--app-id`</md-td>
<md-td>`-a`</md-td>
<md-td>Lark应用的 App ID。

- 如果不指定该参数，则会清除所有应用的 user_access_token。
- 如果指定该参数，则只会清除指定应用的 user_access_token。
</md-td>
<md-td>`-a cli_xxxx`</md-td>
</md-tr>

</md-tbody>
</md-table>
:::



### lark-mcp mcp

`lark-mcp mcp` 支持的命令参数说明如下表所示。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:25%">参数</md-th>
<md-th style="width:15%">简写</md-th>
<md-th style="width:30%">描述</md-th>
<md-th style="width:30%">示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>`--app-id`</md-td>
<md-td>`-a`</md-td>
<md-td>Lark应用的 App ID。</md-td>
<md-td>`-a cli_xxxx`</md-td>
</md-tr>

<md-tr>
<md-td>`--app-secret`</md-td>
<md-td>`-s`</md-td>
<md-td>Lark应用的 App Secret。</md-td>
<md-td>`-s xxxx`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--domain`</md-td>
<md-td>`-d`</md-td>
<md-td>Lark开放平台 API 域名，默认为Lark域名。</md-td>
<md-td>`-d https://open.larksuite.com`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--tools`</md-td>
<md-td>`-t`</md-td>
<md-td>
需要启用的 MCP 工具列表，需参考 [tools](https://github.com/larksuite/lark-openapi-mcp/tree/main/docs) 传入工具对应的 **MCP 工具名称**。

- 支持传入预设工具集。例如传入消息预设工具集（preset.im.default），则会批量启用消息 API。具体使用说明，参见下文 **预设工具集** 章节。
- 支持传入多个（包括单个 API 工具与预设工具集名称），用逗号分隔。
- 全量覆盖，运行该参数后，仅 `-t` 内包含的 API 工具可用。  

</md-td>
<md-td>`-t im.v1.message.create,im.v1.chat.create`</md-td>
</md-tr>

<md-tr>
<md-td>`--tool-name-case`</md-td>
<md-td>`-c`</md-td>
<md-td>MCP 工具注册名称的命名格式，可选值为：
- snake（默认值）
- camel
- dot
- kebab</md-td>
<md-td>`-c camel`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--language`</md-td>
<md-td>`-l`</md-td>
<md-td>
工具语言。可选值为：
- zh：中文
- en（默认值）：英文
</md-td>
<md-td>`-l zh`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--token-mode`</md-td>
<md-td>无</md-td>
<md-td>指定工具启动后调用 API 时所用的 Token 类型。可选值为：

- auto（默认值）：由大模型推理自动选择。
- tenant_access_token：使用应用访问令牌，会过滤掉不支持 tenant_access_token 的 API 工具。
- user_access_token：使用用户访问令牌，会过滤掉不支持 user_access_token 的 API 工具。
</md-td>
<md-td>`--token-mode auto`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--user-access-token`</md-td>
<md-td>`-u`</md-td>
<md-td>用户访问令牌（user_access_token），用于以用户身份调用 API。</md-td>
<md-td>`-u u-xxxx`</md-td>
</md-tr>

<md-tr>
<md-td>`--mode`</md-td>
<md-td>`-m`</md-td>
<md-td>传输模式，可选值为：
  
- stdio（默认值）
- sse
- streamable  
</md-td>
<md-td>`-m sse`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--oauth`</md-td>
<md-td>无</md-td>
<md-td>SSE/Streamable 传输模式下可配置的参数，实现用户身份的自动登录鉴权。</md-td>
<md-td>`lark-mcp mcp -m sse -a <your_app_id> -s <your_app_secret> --oauth`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--host`</md-td>
<md-td>无</md-td>
<md-td>监听主机，默认为 localhost。</md-td>
<md-td>`--host 0.0.0.0`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--port`</md-td>
<md-td>`-p`</md-td>
<md-td>监听端口，默认为 3000。</md-td>
<md-td>`-p 3000`</md-td>
</md-tr>

<md-tr>
<md-td>`--version`</md-td>
<md-td>`-V`</md-td>
<md-td>显示版本号。</md-td>
<md-td>`-V`</md-td>
</md-tr>
  
<md-tr>
<md-td>`--help`</md-td>
<md-td>`-h`</md-td>
<md-td>显示帮助信息。</md-td>
<md-td>`-h`</md-td>
</md-tr>

</md-tbody>
</md-table>
:::


## 参数使用示例

参数需配置在 AI 工具的 MCP 配置文件内，例如 Cursor 工具的 MCP 配置文件为 mcp.json，示例用法如下表所示。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:30%">用法</md-th>
<md-th style="width:70%">示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>基本用法（使用应用身份）</md-td>
<md-td>通过 `-a`、`-s` 配置应用的 App ID、App Secret。
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>"
      ]
    }
  }
}
```
</md-td>
</md-tr>

<md-tr>
<md-td>指定工具启动后调用 API 时所用的 Token 类型。
</md-td>
<md-td>通过 `--token-mode` 指定 Token 类型。可指定的值有：
  
- auto（默认值）：由大模型推理自动选择。
- tenant_access_token：使用应用访问令牌，会过滤掉不支持 tenant_access_token 的 API 工具。
- user_access_token：使用用户访问令牌，会过滤掉不支持 user_access_token 的 API 工具。
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "--token-mode",
        "tenant_access_token"
      ]
    }
  }
}
```
</md-td>
</md-tr>  

<md-tr>
<md-td>使用用户访问令牌（user_access_token）</md-td>
<md-td>
如果需要以特定用户身份调用API，可以通过 `-u` 指定用户访问令牌（user_access_token）来实现。
  
<md-alert>
user_access_token 获取方式参见 [常见问题](/document/uAjLw4CM/ukTMukTMukTM/mcp_integration/use_cases) 中的 **如何快速获取应用对应的用户访问凭证（user_access_token）**。
</md-alert>
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "-u",
        "<your_user_access_token>"
      ]
    }
  }
}
```
</md-td>
</md-tr>
  
<md-tr>
<md-td>设置 MCP 工具语言为中文</md-td>
<md-td>
通过 `-l` 指定工具语言。
  
<md-alert>
设置语言为中文（zh）可能会消耗更多的 token，如果在与大模型集成时遇到 token 限制问题，可以考虑使用默认的英文（en）。
</md-alert>
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "-l",
        "zh"
      ]
    }
  }
}
```
</md-td>
</md-tr>

<md-tr>
<md-td>设置 MCP 工具名称格式为驼峰式</md-td>
<md-td>
通过 `-c` 设置工具名称格式。  
  
<md-alert>
通过设置工具名称格式，可以改变工具在 MCP 中注册的调用名称格式。例如，`im.v1.message.create` 在不同格式下的表现形式：
- snake 格式（默认）：`im_v1_message_create`
- camel 格式：`imV1MessageCreate`
- kebab 格式：`im-v1-message-create`
- dot 格式：`im.v1.message.create`
</md-alert>
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "-c",
        "camel"
      ]
    }
  }
} 
```
</md-td>
</md-tr>

<md-tr>
<md-td>指定自定义域名</md-td>
<md-td>
如果你使用的是 Lark 国际版或自定义域名，可以通过`-d`参数指定，`<URL>` 需要替换为具体的域名：
  
- Lark 国际版域名为：https://open.larksuite.com
- 自定义域名示例：https://open.your-ka-domain.com
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "-d",
        "<URL>"
      ]
    }
  }
}
```
</md-td>
</md-tr>
  
<md-tr>
<md-td>设置传输模式</md-td>
<md-td>
MCP 工具支持两种传输模式：
  
1. **stdio模式（默认/推荐）**：`-m` 取值 `stdio`，该模式适用于与 AI 工具集成，通过标准输入输出流进行通信。
  
    ```json
    {
      "mcpServers": {
        "lark-mcp": {
         "command": "npx",
          "args": [
            "-y",
            "@larksuiteoapi/lark-mcp",
            "mcp",
            "-a",
            "<your_app_id>",
            "-s",
            "<your_app_secret>",
            "-m",
            "stdio"
          ]
        }
      }
    }
    ```

2. **SSE 模式**：`-m` 取值 sse，提供基于 Server-Sent Events 的 HTTP 接口，适用于 Web 应用或需要网络接口的场景。
  
	启动后，SSE 端点将可在 `http://<host>:<port>/sse` 访问。

  	1. 在 MCP 配置文件中，增加 sse URL，并保存文件。
    
  		如下图示例 URL：`http://localhost:3000/sse`
  
		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eefd1a8670c1f866be4935b5bee31679_7Ye30vyv3B.png?height=822&lazyload=true&maxWidth=300&width=1316)
  
    2. 在终端命令行执行以下命令。
  
  		```bash
  		lark-mcp mcp -a cli_xxxxxxx -s dfl4xxxx -m sse
  		```
</md-td>
</md-tr>

<md-tr>
<md-td>启用特定的 API 工具</md-td>
<md-td>
默认情况下，MCP 服务启用常用 API，如需启用其他工具或仅启用特定 API，可以通过 `-t` 参数指定
- 需参考 [tools](https://github.com/larksuite/lark-openapi-mcp/tree/main/docs) 传入工具对应的 **MCP** **工具名称**。

- 支持传入预设工具集。例如传入消息预设工具集（preset.im.default），则会批量启用消息 API。具体使用说明，参见下文 **预设工具集** 章节。
  
- 支持传入多个（包括单个 API 工具与预设工具集名称），用逗号分隔。
  
<md-alert>
该方式为全量覆盖，会将 **MCP 工具默认启用的 OpenAPI** 给覆盖掉，只启用 `-t` 内包含的 API 工具。
</md-alert>
  
```json
{
  "mcpServers": {
    "lark-mcp": {
     "command": "npx",
      "args": [
        "-y",
        "@larksuiteoapi/lark-mcp",
        "mcp",
        "-a",
        "<your_app_id>",
        "-s",
        "<your_app_secret>",
        "-t",
        "im.v1.message.create,im.v1.message.list,im.v1.chat.create"
      ]
    }
  }
} 
```
</md-td>
</md-tr>
  
<md-tr>
<md-td>使用环境变量代替命令行参数</md-td>
<md-td>
```
# 设置环境变量
export APP_ID=<your_app_id>
export APP_SECRET=<your_app_secret>

# 启动服务（无需指定 -a 和 -s 参数）
lark-mcp mcp 
```
</md-td>
</md-tr>

</md-tbody>
</md-table>
:::
  
## 预设工具集
  
:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:30%">工具集名称</md-th>
<md-th style="width:70%">包含的 API 工具</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>preset.default</md-td>
<md-td>OpenAPI MCP 默认启用的工具集，包含：
  
- im.v1.chat.create：[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)
- im.v1.chat.list：[获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)
- im.v1.chatMembers.get：[获取群成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get)
- im.v1.message.create：[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)
- im.v1.message.list：[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)
- bitable.v1.app.create：[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)
- bitable.v1.appTable.create：[新增一个数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create)
- bitable.v1.appTable.list：[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)
- bitable.v1.appTableField.list：[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)
- bitable.v1.appTableRecord.search：[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)
- bitable.v1.appTableRecord.create：[新增记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create)
- bitable.v1.appTableRecord.update：[更新记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/update)
- docx.v1.document.rawContent：[获取文档纯文本内容](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content)
- docx.builtin.import：导入文档，包括上传素材/文件、创建导入任务、查询导入任务结果三步骤，详情参见[导入文件概述](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide#461aa643)
- docx.builtin.search：[搜索云文档](/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN)
- drive.v1.permissionMember.create：[增加协作者权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)
- wiki.v2.space.getNode：[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)
- wiki.v1.node.search：[搜索 Wiki](/document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki)
- contact.v3.user.batchGetId：[通过手机号或邮箱获取用户 ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)
</md-td>
</md-tr>
  
<md-tr>
<md-td>preset.light</md-td>
<md-td>精简的 API 工具集，包含：
  
- im.v1.chat.search：[搜索对用户或机器人可见的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search)
- im.v1.message.create：[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)
- im.v1.message.list：[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)
- bitable.v1.appTableRecord.search：[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)
- bitable.v1.appTableRecord.batchCreate：[新增多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create)
- docx.v1.document.rawContent：[获取文档纯文本内容](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content)
- docx.builtin.import：导入文档，包括上传素材/文件、创建导入任务、查询导入任务结果三步骤，详情参见[导入文件概述](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide#461aa643)
- docx.builtin.search：[搜索云文档](/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN)
- wiki.v2.space.getNode：[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)
- contact.v3.user.batchGetId：[通过手机号或邮箱获取用户 ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)
</md-td>
</md-tr>

<md-tr>
<md-td>preset.im.default</md-td>
<md-td>消息与群组 API 工具集，包含：

- im.v1.chat.create：[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)
- im.v1.chat.list：[获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)
- im.v1.chatMembers.get：[获取群成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get)
- im.v1.chatMembers.create：[将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)
- im.v1.message.create：[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)
- im.v1.message.list：[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)
</md-td>
</md-tr>
  
<md-tr>
<md-td>preset.base.default</md-td>
<md-td>多维表格的 API 工具集，包含：
- bitable.v1.app.create：[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)
- bitable.v1.appTable.create：[新增一个数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create)
- bitable.v1.appTable.list：[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)
- bitable.v1.appTableField.list：[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)
- bitable.v1.appTableRecord.search：[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)
- bitable.v1.appTableRecord.create：[新增记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create)
- bitable.v1.appTableRecord.update：[更新记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/update)
</md-td>
</md-tr>
  
<md-tr>
<md-td>preset.base.batch</md-td>
<md-td>多维表格批量处理 API 工具集，包含：
- bitable.v1.app.create：[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)
- bitable.v1.appTable.create：[新增一个数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create)
- bitable.v1.appTable.list：[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)
- bitable.v1.appTableField.list：[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)
- bitable.v1.appTableRecord.search：[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)
- bitable.v1.appTableRecord.batchCreate：[新增多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create)
- bitable.v1.appTableRecord.batchUpdate：[更新多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_update)
</md-td>
</md-tr>
  
<md-tr>
<md-td>preset.doc.default</md-td>
<md-td>云文档 API 工具集，包含：

- docx.v1.document.rawContent：[获取文档纯文本内容](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content)
- docx.builtin.import：导入文档，包括上传素材/文件、创建导入任务、查询导入任务结果三步骤，详情参见[导入文件概述](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide#461aa643)
- docx.builtin.search：[搜索云文档](/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN)
- drive.v1.permissionMember.create：[增加协作者权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)
- wiki.v2.space.getNode：[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)
- wiki.v1.node.search：[搜索 Wiki](/document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki)
</md-td>
</md-tr>

<md-tr>
<md-td>preset.task.default</md-td>
<md-td>任务 API 工具集，包含：
  
- task.v2.task.create：[创建任务](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/create) 
- task.v2.task.patch：[更新任务](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/patch)
- task.v2.task.addMembers：[添加任务成员](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_members)
- task.v2.task.addReminders：[添加任务提醒](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_reminders)
</md-td>
</md-tr>
  
<md-tr>
<md-td>preset.calendar.default</md-td>
<md-td>日历 API 工具集，包含：
  
- calendar.v4.calendarEvent.create：[创建日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/create)
- calendar.v4.calendarEvent.patch：[更新日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/patch)
- calendar.v4.calendarEvent.get：[获取日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get)  
- calendar.v4.freebusy.list：[查询主日历日程忙闲信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/list) 
- calendar.v4.calendar.primary：[查询主日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)
</md-td>
</md-tr>
  
</md-tbody>
</md-table>
:::
