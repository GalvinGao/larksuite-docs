---
document_id: '7026663896463491077'
directory_id: '7002892512470630406'
title: 步骤四：发送消息卡片
full_path: /home/build-a-beautiful-message-card-in-5-minutes/send-message-card
breadcrumb:
- Home
- Quick intro to message card
- 'Step 4: Send message card'
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:12Z
source_url: https://open.larksuite.com/document/home/build-a-beautiful-message-card-in-5-minutes/send-message-card
---

# 步骤四：发送消息卡片

在本步骤，你将调用发送消息接口，发送卡片消息到群聊。

## 步骤一：发送消息
:::html
<md-td>
1. 登录[API调试台](https://open.larksuite.com/api-explorer)。

2. 在左上角单击**切换应用**，然后在弹出的对话框中选择步骤一创建的应用，最后单击**确定**完成切换。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/451b1c525665c70a0d7967c29e309ec3.png" style="width:70%"/>

3. 在左侧查看鉴权凭证栏，单击获取 **tenant_access_token** 和 **user_access_token** 。

	如下图所示，在获取 **user_access_token** 时，你需要将API调试台添加为当前应用的可信的重定向地址。
   
   <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a56858cd75196d3b58073d906150f83f.png?lazyload=true&width=2548&height=1304" style="width:70%"/>

4. 调用 [通过手机号或邮箱获取用户 ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id) 接口，获取用户的open_id。
   
   1. 在API列表中依次选择 **通讯录** > **用户** > **通过手机号或邮箱获取用户ID** 。
   
   2. 单击 **权限配置**，然后选中未开通的权限，单击 **批量开通**。
   
		<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/df9e0a538f8f4979557ef9602e40d751.png" style="width:70%"/>
   
   3. 单击 **查询参数**，修改 **user_id_type**为 `open_id`。
   
   4. 单击 **请求体**，在请求 JSON 中填入以下内容并修改对应的参数信息，最后单击右上角 **开始调试**。
  	
  		需要修改`mobiles`的值为你的手机号。
      
      ```json
      {
        "mobiles": [
          "180xxx120" 
        ]
      }
      ```
   
   5. 调用成功后记录下返回结果中 **user_id** 的值，该值就是用户的 **open_id**。
      
      后续步骤中将使用该值作为群组的      **owner_id**      。
      
      <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8a76b7eac37c6b282d1a4932795c3025_nfUIHIWnsc.png?lazyload=true&width=1912&height=1288" style="width:70%"/>

5. 调用[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)接口，创建一个用于测试的群聊。
   
   1. 在API列表中依次选择 **群组** > **群组管理** > **创建群**。
   
   2. 单击 **权限配置**，然后选中未开通的权限，单击 **批量开通**。
      
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f0750426d9159624373ff6a2abaf034f.png?lazyload=true&width=1924&height=1230" style="width:70%"/>
   
   3. 单击 **查询参数**，然后修改 **set_bot_manager** 参数为 `true`。
   
   4. 单击**请求体**，在请求 JSON 中填入以下内容。
      
       * 修改 **name** 的值为你的群聊名称。
      
       * 修改 **owner_id** 的值为**步骤4.e**获取的用户 **open_id**。
      
      ```json
      {
        "name": "测试群名称",     // 群聊名称
        "owner_id":"ou_00bdxxx5e68"  // 群主的open_id
      }
      ```
   
   5. 调用成功后记录下群的 **chat_id**。
  
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/85c718e6a89b988c2dbefb79e1b5d52e.png?lazyload=true&width=1918&height=1290" style="width:70%"/>

6. 参考以下步骤，调用[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口，发送卡片消息到群。
   
   1. 在API列表中依次选择 **消息** > **消息管理** > **发送消息**。
   
   2. 单击 **查询参数**，修改 **receive_id_type** 参数为 `chat_id`。
   
   3. 单击 **请求体**，在请求 JSON 中填入以下内容并修改对应的参数信息。单击右上角 **发起调用**。
      
      ```JSON
      {
        "receive_id": "oc_7bc568xxx24e",   // 群chat_id
        "msg_type": "interactive", // 消息类型，interactive为卡片
        // template_id为消息卡片ID。
        "content": "{\"type\": \"template\", \"data\": {\"template_id\": \"ctp_AAYHcTkBZXa0\"}}"
      }
      ```

      调用成功后的结果类似如下：
      
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/ecc478cb5f3a77edbd09ebe60527597e.png" style="width:70%"/>

## 步骤二：验证
1. 打开Lark客户端，找到上一步中创建的群，单击群名称打开聊天窗口。
   
   如下图所示，卡片消息已经成功发送到群聊中。
   
   <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/3adc5434fdbad3d4e05129b2d0272ca3.png" style="width:70%"/>
2. 依次单击下方三个按钮，查看对应功能是否符合预期。
   
   * 跳转按钮效果如下：
      
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f43d9166f0898a3edd5569f1b753cbbf.gif" style="width:70%"/>
   
   * 二次确认效果如下：
      
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/5e2c17292739024ccf10137862248d5b.gif" style="width:70%"/>
   
   * 回传交互效果如下：
      
      <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/5b7910e82966b459ebad935a98574c6b.gif" style="width:70%"/>
      
      点击回传交互按钮后，你的服务端将收到包含以下请求体的 POST 请求。
      
      ```JSON
      {
          "open_message_id": "om_a7acxxxc58e4",
          "tenant_key": "109xxx758",
          "open_id": "ou_d72axxx5454622e2e",
          "user_id": "4bxxx9f8",
          "action": {
              "value": {
                  "key": "value",
                  "key1": "value1"
              },
              "tag": "button"
          },
          "open_chat_id": "oc_43aaaxxxe6bcd",
          "token": "c-cfffac0129xxx3eb3f8f5"
      }
      ```
</md-td>
:::
:::note
如果需要对用户操作进行反馈，可参考[步骤五：响应用户操作](/document/home/build-a-beautiful-message-card-in-5-minutes/feedback-on-user-behavior)。
:::

