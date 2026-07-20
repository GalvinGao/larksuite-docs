---
document_id: '7026663896463589381'
directory_id: '7002892512470630406'
title: 步骤三：配置卡片请求地址
full_path: /home/build-a-beautiful-message-card-in-5-minutes/add-interaction
breadcrumb:
- Home
- Quick intro to message card
- 'Step 3: Configure card request address'
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:08Z
source_url: https://open.larksuite.com/document/home/build-a-beautiful-message-card-in-5-minutes/add-interaction
---

# 步骤三：配置卡片请求地址
在用户对消息卡片进行操作时，如果你需要对用户操作进行响应，那么你还需要配置消息卡片请求网址。用户完成卡片操作之后，Lark开放平台会向将用户操作信息以POST请求的方式发送到卡片请求网址。本文介绍如何配置消息卡片请求网址。
## 操作步骤
:::html
<md-td>
1. 登录[开发者后台](https://open.larksuite.com/app)， 然后进入应用详情页。

2. 在应用详情页，依次选择 **添加应用能力** > **按能能力添加** > **机器人**，然后单击 **添加能力**。

3. 在机器人配置页面，单击机器人配置后方的编辑按钮。

4. 在消息卡片请求网址下方的输入框中，填写的你卡片回调地址。
  
	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e40a8b8ed61d02491f922b18704a23de.png?lazyload=true&width=2624&height=1608" style="width:70%"/>
  
 	 在你点击 **验证** 或 **保存** 时，开放平台会向你配置的请求地址发送一个 `application/json` 格式的POST请求，用于验证你所配置地址的合法性。

	请求示例如下：
    
    ```JSON
    {
         "challenge": "1b6aef1a-401f-406a-be41-f48911e00be7", 
         "type": "url_verification", 
         "token": "qjSwzC****4T1uhJ"
    }
    ```

 	 参数说明如下：

    | **参数** | **类型** | **示例值** | **说明** |
    | --- | --- | --- | --- |
    | challenge | String | 1b6aef1a-401f-*** | 你需要在响应中原样返回的值。 |
    | type | String | url_verification | 事件类型。 <br> 当前请求中固定值为 `url_verification` 表示当前请求为验证URL合法性。 |
    | token | String | qjSwzC****4T1uhJ | 应用验证标识，你可以根据此 Token 验证推送的事件是否属于当前应用。 <br> 可以在[开发者后台](https://open.larksuite.com/app) > **事件订阅** 页面，查看和重置该值。 |

	你需要在 3s 内将`challenge`值（JSON格式）原样返回给Lark开放平台，否则请求地址验证将失败。

	返回示例如下：
    ```JSON
    { 
        "challenge": "1b6aef1a-401f-406a-be41-f48911e00be7"
    }
    ```

5. 验证通过后，单击 **保存** 完成配置。

6. 卡片回调地址配置成功后，在左侧导航栏，单击 **权限与管理**，然后添加以下权限。
  	<md-perm name="im:message" desc="获取与发送单聊、群组消息">获取与发送单聊、群组消息</md-perm>

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/3995fa2a43128e99ded281f9cd12c328.png?lazyload=true&width=3206&height=1118" style="width:70%"/>
</md-td>
:::
