---
document_id: '7491256897678721036'
directory_id: '7393239659646533638'
title: 快速入门
full_path: /uAjLw4CM/ukzMukzMukzM/development-link-preview/quick-start
breadcrumb:
- Developer Guides
- Development link preview
- Quick start
document_type: GuideDocumentType
updated_at: 2025-04-11T02:06:47Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukzMukzMukzM/development-link-preview/quick-start
---

# 快速入门

本文提供完整的链接预览配置流程，供你参考并上手体验链接预览功能。

## 准备工作

- 已了解链接预览功能的实现方式与配置流程，详情参见[链接预览开发指南](/document/uAjLw4CM/ukzMukzMukzM/development-link-preview/link-preview-development-guide)。

-   本地已配置 [Node.js](https://nodejs.org/en) 开发环境，或者 [Go](https://go.dev/) 开发环境。
    
    本文提供了 [Node.js 示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/096e72afaf3b3086910dbab3217e86d4_O3xgFH04vt.js) 和 [Go 示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ee10b1d29a2548e410976223949d4dc3_5gNzacrU1c.zip)，你可以根据本地开发环境选择使用。


## 步骤一：创建并配置应用

1. 登录[开发者后台](https://open.larksuite.com/app)。
2. 在 **企业自建应用** 页面，点击 **创建企业自建应用**。
    
    本文以自建应用为例介绍配置流程。你也可以选择创建商店应用，其中的链接预览配置流程与自建应用相同。配置商店应用的具体操作，参见[流程概述](/document/uMzNwEjLzcDMx4yM3ATM/ugzNwEjL4cDMx4CO3ATM)。
3. 配置应用的名称、描述与图标，并点击 **保存**。
    
    例如，创建名为 `Card & URL Demo` 的应用，描述与图标自定义配置即可。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0d13c3d4aaaecdfa20b081f7ad72d1ae_T80aq4Cf78.png?height=1392&lazyload=true&maxWidth=400&width=1184)



## 步骤二：构建消息卡片

你需要构建一张消息卡片，后续用于绑定链接预览。了解消息卡片可参见[消息卡片概述](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)。

1. 登录[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder).
2. 在页面顶部点击 **新建卡片**。
3. 选择一个卡片模板。

	例如选择 **活动通知** 模板。

4. 配置卡片名称、卡片组，并点击 **保存**。
    
    本示例中，创建名为 `CardDemo` 卡片，卡片组选择 `我的卡片`。

5. 创建后，在卡片编辑页面右上角点击 **保存并发布**。

	你也可以自行修改卡片内容再保存。

6. 在 **提示** 对话框，阅读提示信息并点击 **发布**。

7. 在卡片预览区域，获取卡片 ID。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1a6976400c4d3e63c9afa0690bff094b_GIvWG5m8uU.png?height=560&lazyload=true&maxWidth=400&width=1038)

## 步骤三：下载并运行示例代码

使用本文提供的示例代码包，在本地构建一个业务服务器，用于处理来自Lark开放平台的回调请求。

### Node.js 代码包

1. 下载[示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/096e72afaf3b3086910dbab3217e86d4_O3xgFH04vt.js)。
    
    ```bash
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/096e72afaf3b3086910dbab3217e86d4_O3xgFH04vt.js -o url_preview.js
    ```
2. 下载完成后，进入示例代码所在的文件目录。
3. 打开 url_preview.js 文件，修改卡片 ID 与卡片版本号。
    
    使用你常用的打开文件方式即可。例如，在终端内使用 `vi/vim` 命令打开文件，或使用 VSCode 等本地开发工具打开文件。
        
	- 找到 `template_id`，将代码中的示例值，修改为你构建的Lark卡片的真实 ID。
        
	- 找到 `template_version_name`，将代码中的示例值改为空值。

	- 如果你的Lark卡片配置了卡片变量，则需要通过 `template_variable` 字段为变量赋值。
        
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e5846ef22aeae34d01070fe3d23f27ed_5mCH5ro1aW.png?height=848&lazyload=true&maxWidth=600&width=1462)

4. 在本地使用命令行工具，进入示例代码所在目录并运行以下命令，启动服务并监听 3000 端口。
   
   ```bash
   node url_preview.js
   ```

### Go 代码包

1. 下载 [Go 示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ee10b1d29a2548e410976223949d4dc3_5gNzacrU1c.zip)。
    
    ```
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ee10b1d29a2548e410976223949d4dc3_5gNzacrU1c.zip -o preview_example.zip
    ```
2. 在本地解压已下载的代码包。
3. 在代码包中打开 preview.go 文件，找到 `url_preview` 字段并根据实际情况修改回调结构。
    
    本示例中，通过 [GoLand](https://www.jetbrains.com/go/) 工具打开并配置代码包。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9315b7b1913f078cdef8e4f7b4ca1240_Qhjc9l3EVM.png?height=1338&lazyload=true&maxWidth=600&width=2848)
    
    回调结构配置说明：
    
    - 找到 `TemplateID`，将代码中的示例值，修改为你构建的Lark卡片的真实 ID。
    - 找到 `VersionName`，将代码中的示例值改为空值。
    - 如果你的Lark卡片配置了卡片变量，则需要通过 `Variable` 为变量赋值。

4. 在 main.go 中运行 `main` 函数，启动服务并监听 **3000** 端口。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6b5bc3a6826c52d0efbc6d4cc164c56a_dvEXnVKVhd.png?height=1496&lazyload=true&maxWidth=600&width=2872)

### 获取本地公网地址

:::warning
**注意：**<br>
- 如果本机有公网地址，可跳过本步骤。
- 如果本机没有公网地址，本教程为了方便实现，使用了反向代理工具（[ngrok](https://ngrok.com/download)）完成内网穿透，暴露本地服务的公网访问入口。**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策。**
- **测试完成后，如需正式发布应用，你需要修改为真实有效的公网地址。**
:::   

1. 注册并安装 [ngrok](https://ngrok.com/download)，按照官方指引完成安装。
2. 在个人的 [dashboard 页面](https://dashboard.ngrok.com/get-started/your-authtoken) 中，获取 Authtoken。
        
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3a6d48a86bbd55342a81f80e43561204_dLGeBfcu8M.png?height=624&lazyload=true&maxWidth=600&width=2480)

3. 在本地依次运行以下命令，获得公网 URL。
        
    ```bash
    ngrok authtoken "token" // token需替换为实际值
    ngrok http 3000
    ```
        
    成功运行结果如下图：
        
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4c5eeb0e8ae06c9df51c0849e8923b35_mwLSArWG5Z.png?height=882&lazyload=true&maxWidth=600&width=2490)

4. 保存本地公网地址。
    
    后续用于配置链接预览回调。

## 步骤四：配置链接预览

1. 登录[开发者后台](https://open.larksuite.com/app)。
2. 进入应用详情页，并在左侧导航栏选择 **应用能力** > **添加应用能力**。
3. 在 **按能力添加** 页签中，找到并添加 **链接预览**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4c94b60278e56bc39517852df43ce90d_w7B18i0ZHJ.png?height=1238&lazyload=true&maxWidth=600&width=2254)
4. 进入 **链接预览** 功能页，在 **注册需要自定义预览的 URL 规则** 区域，点击 **添加 URL 规则**。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d5d8775abf2b4f577ad520703673731c_5UXfuzs5TX.png?height=850&lazyload=true&maxWidth=600&width=1914)

    本示例中，添加一条 `example.com/*` 规则，后续在Lark客户端内发送的链接如果命中该规则，则会实现链接预览。
    
   
	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a10283c8255420ae03f4afd7343ba2a2_DbVpWeb306.png?height=270&lazyload=true&maxWidth=600&width=1253)
5. 配置应用回调。
    
    1. 在 **配置订阅链接预览回调** 区域，点击 **去配置**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ddedf8a7e3375ae62ba95b9dfdb67610_tY84bAtHOZ.png?height=296&lazyload=true&maxWidth=600&width=1710)
        
    2. 在 **事件与回调** 页面的 **回调配置** 页签中，点击 **订阅方式** 右侧的编辑按钮。

		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/68349c659dadb165ea75b2ad43515b4f_bUNJt4nInw.png?height=536&lazyload=true&maxWidth=600&width=1758)
    3. 将运行示例代码获取到的公网地址，配置在 **请求地址** 输入框，并点击 **保存**。
        
        你需要填写本地获取到的真实地址。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a2401279e43e9bc22ffa1fb13537bab5_kjZrnomlBi.png?height=560&lazyload=true&maxWidth=600&width=1658)
    4. 在页面底部的 **已订阅的回调** 区域右侧，点击 **添加回调**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/efe2f1373dd4451829d07ecea755f4ae_zX5jWE9JOC.png?height=1142&lazyload=true&maxWidth=600&width=2244)
    5. 在对话框左侧导航栏点击 **链接预览**，选中 **拉取链接预览数据**，并点击 **确认添加**。

		回调详情参考[拉取链接预览数据](/document/uAjLw4CM/ukzMukzMukzM/development-link-preview/pull-link-preview-data-callback-structure)。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/40f5c450d5642d2ee36197cf7104a683_1WoH6IjXkC.png?height=1192&lazyload=true&maxWidth=600&width=1666)
    6. 返回应用的 **链接预览** 功能页，查看 **配置订阅链接预览回调** 的状态为 **已配置**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5b1f041413f0d42b69d111a3b5656a08_qNa5iLdFod.png?height=432&lazyload=true&maxWidth=600&width=1091)
        
6. 发布应用，使以上配置生效。

	1. 在 **版本管理与发布** 页面，点击 **创建版本**。
	2. 在 **版本详情** 页面，根据页面提示依次配置版本号、能力、说明等信息，并在页面底部点击 **保存**。
	3. 在弹窗内确认提交应用审核。
	
    	你需要等待企业管理员通过应用审核，然后应用会自动发布。

## 步骤五：体验效果

登录Lark客户端，分别在以下消息场景中体验链接预览效果。

:::warning
**注意**：链接需要以 https/http 开头（即使用 HTTPS/HTTP 协议的链接），才可以成功实现预览效果。
:::

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:60%">场景</md-th>
<md-th style="width:40%">图示</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>单独发送 `https://example.com/path`</md-td>
<md-td>

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/10a85b69dd41eea0e48766ab951801b4_xO9s5mVFGj.png?height=776&lazyload=true&width=958)  
</md-td>
</md-tr>

<md-tr>
<md-td>富文本消息包含 `https://example.com/path`</md-td>
<md-td>
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cfc41a0432fb907fc0661ed172cb9ea9_LwgJQnGUxz.png?height=782&lazyload=true&width=956)  
</md-td>
</md-tr>
  
<md-tr>
<md-td>群置顶消息 `https://example.com/path`</md-td>
<md-td>
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7b3c8d54aa57bc750c0dc20152141474_9xf0Y2vQ62.png?height=185&lazyload=true&width=1280)  
</md-td>
</md-tr>

</md-tbody>
</md-table>
:::

