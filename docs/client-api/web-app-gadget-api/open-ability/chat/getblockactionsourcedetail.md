---
document_id: '6965379541105098757'
directory_id: '6907567269107597314'
title: getBlockActionSourceDetail
full_path: /getBlockActionSourceDetail
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- getBlockActionSourceDetail
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:23Z
source_url: https://open.larksuite.com/document/getBlockActionSourceDetail
---

# getBlockActionSourceDetail(Object object)

支持从 [消息快捷操作](/document/ukzMukzMukzM/ugDN1YjL4QTN24CO0UjN) 点击进入应用后，获取应用模块对应业务的详细信息。

:::html
<md-alert type="tip">
小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )
</md-alert>
:::


## 支持说明
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                triggerCode
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                进入应用时，getHostLaunchQuery获取的参数

**示例值**：c-66a6cd5a2340665fc49e74e4e5c49290

**最小长度**：`1`  字符
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出


`success`返回对象的扩展属性：

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                bizType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                业务类型

**可选值**：
- `message`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                content
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                查询的message对象
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    actionTime
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                Action发生的时间戳、单位秒
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    messages
                </md-text>
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                message列表
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    messageType
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                消息类型

**可选值**：
- `text`：文本消息
- `image`：图片消息
- `post`：富文本消息
- `media`：视频消息，支持版本[3.47+](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)
- `file`：文件消息，支持版本[3.47+](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)
- `interactive`：[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)消息，支持版本[3.47+](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)
- `unsupport`：暂不支持的消息类型
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    sender
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                message发送者对象
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;&emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    name
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                message发送者名字，按调用时客户端语言提供
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;&emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    open_id
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户 [open_id](/document/home/user-identity-introduction/open-id)
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    createTime
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                message创建的时间戳、单位秒
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    support
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                是否是支持的消息类型
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    content
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                消息内容：json字符串、文本内容
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    status
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                消息状态是否有效
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openChatId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                触发操作会话的  [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
              
<md-alert type="tip" icon="none">
Lark[V3.40.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openMessageId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                触发操作的消息 [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
             
<md-alert type="tip" icon="none">
Lark[V3.40.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>      
        
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码


```js
tt.getHostLaunchQuery({
    success(res) {
        if (res.launchQuery) {
            let json = JSON.parse(res.launchQuery)
            if (json && json.__trigger_id__) {
                // getHostLaunchQuery获取triggerCode的方法
                tt.getBlockActionSourceDetail({
                    triggerCode:json.__trigger_id__,
                    success(res) {
                        console.log(JSON.stringify(res));
                    },
                    fail(res) {
                        console.log(`getBlockActionSourceDetail fail:${JSON.stringify(res)}`);
                    }
                })
            } 
        } else {
            console.log(`getHostLaunchQuery fail:${JSON.stringify(res)}`);
        }
    },
});
```

`success`返回对象示例：

不同消息类型的返回Message参数示例
#### text 文本消息
```json
{
    "messageType":"text",
    "sender":{
        "name":"剑豪",
        "open_id":"ou_bbb89d41d088803ef89f1a798121ac39"
    },
    "createTime":1603175461,
    "support":true,
    "openChatId": "oc_xxxxx",
    "openMessageId": "om_xxxx",
    "content":"{\"text\":\"123\"}",
    "status":true
} 
``` 
#### post 富文本消息
```json
{
    "messageType":"post",
    "sender":{
        "name":"剑豪",
        "open_id":"ou_bbb89d41d088803ef89f1a798121ac39"
    },
    "createTime":1602911632,
    "support":true,
    "openChatId":"oc_xxxxx",
    "openMessageId":"om_xxxx",
    "content":"{\"title\":\"来来来\",\"content\":[{\"attrs\":[{\"tag\":\"at\",\"open_id\":\"ou_bbb89d41d088803ef89f1a798121ac39\",\"text\":\"@剑豪\"},{\"tag\":\"text\",\"text\":\" \"}]},{\"attrs\":[{\"tag\":\"img\",\"url\":\"xxximageUrl\"}]},{\"attrs\":[{\"tag\":\"text\",\"text\":\"光谷六路十五号\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"text\",\"text\":\"啦啦啦啦\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"},{\"tag\":\"unknown\",\"text\":\"\"}]}]}",
    "status":true
}
``` 

#### image 图片消息
```json
{
    "messageType":"image",
    "sender":{
        "name":"XXX",
        "open_id":"XXXXID"
    },
    "createTime":1602584487,
    "support":true,
    "openChatId":"oc_xxxxx",
    "openMessageId":"om_xxxx",
    "content":"{\"url\":\"https://s3-imfile.feishucdn.com/static-resource/v1/f8836d60-07cb-4fd4-92ce-01e54952f79g~?image_size=72x72&cut_type=&quality=&format=png&sticker_format=.webp\"}",
    "status":true
}
``` 

:::note
其中，url 为图片的下载链接，关于**如何通过URL获取图片详情**，参见文末"[其他说明](/document/getBlockActionSourceDetail#26591811)"
:::

#### media 视频消息
```json
{
    "messageType":"media",
    "sender":{
        "name":"XXX",
        "open_id":"XXXXID"
    },
    "createTime":1602584487,
    "support":true,
    "openChatId":"oc_xxxxx",
    "openMessageId":"om_xxxx",
    "content":"{\"file_url\":\"xxx\",\"image_url\":\"​xxx\",\"duration\":\"xxx\"}",
    "status":true
}
``` 
:::note
file_url 为视频消息中视频文件的下载链接，image_url 为视频预览封面的下载链接，duration 为视频的时长，单位为 ms；下载方式可详见文末[其他说明](/document/getBlockActionSourceDetail#26591811)。
:::

#### file 文件消息
```json
{
    "messageType":"file",
    "sender":{
        "name":"XXX",
        "open_id":"XXXXID"
    },
    "createTime":1602584487,
    "support":true,
    "openChatId":"oc_xxxxx",
    "openMessageId":"om_xxxx",
    "content":"{\"file_url\":\"xxx\"}",
    "status":true
}
``` 
:::note
file_url 为文件消息中对应文件的下载链接，下载方式可详见文末[其他说明](/document/getBlockActionSourceDetail#26591811)。
:::

#### interactive [消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)消息
```json
{
    "messageType":"interactive",
    "sender":{
        "name":"XXX",
        "open_id":"XXXXID"
    },
    "createTime":1602584487,
    "support":true,
    "openChatId":"oc_xxxxx",
    "openMessageId":"om_xxxx",
    "content":"{\"title\":\"xxx\",\"elements\":[[{\"tag\":\"at\",\"user_id\":\"xxx\",\"user_name\":\"xxx\"},{\"tag\":\"text\",\"text\":\"xxx\"}],[{\"tag\":\"img\",\"image_key\":\"xxx\"},{\"tag\":\"a\",\"href\":\"xxx\",\"text\":\"xxx\"}],...]}",
    "status":true
}
``` 

#### unsupport 暂不支持的消息类型

```json
{
    "messageType":"unsupport",
    "sender":{
        "name":"剑豪",
        "open_id":"ou_bbb89d41d088803ef89f1a798121ac39"
    },
    "createTime":1603357833,
    "support":false,
    "openChatId": "oc_xxxxx",
    "openMessageId": "om_xxxx",
    "content":"[该消息类型暂不支持快捷操作]",
    "status":true
}
 
``` 



## 错误码


`fail`返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码，具体错误码列表参见：
| 错误码         | 错误信息           |  描述        |
| --------- | --------------- | --------- |
|`42301` | `triggerCode is empty` |  triggerCode为空 |
|`42302` | `invalid triggerCode` |  triggerCode不合法 |
|`42303` | `not login` |  未登录 |
|`42304` | `get block action failed` |  获取message信息失败 |
|`42305` | `service is not valid` |  依赖服务不可用 |


## 其他说明
> 如何以URL获取图片/视频/文件的详情。

**1. 通过接口获取URL**
以图片为例，json结构如下：

```json 
{
    "url":"https://open.larksuite.com/open-apis/image/v4/get?image_key=img_axxxxxxxxxxx9g&operator=app"
}
``` 

**2. 根据获取到的URL下载资源**
- 请求

| 基本 |     |
| --------- | --------------- | -------   | ----------- | --------- |
| HTTP URL |  **下载视频/文件**： https://open.larksuite.com/open-apis/im/v1/files/file_key ; **下载图片**： https://open.larksuite.com/open-apis/image/v4/get?image_key=xxx&operator=app|
| HTTP Method |  GET 请求|

- 请求头

| 参数         | 数据类型           | 必填        | 描述         | 实例       |
| --------- | --------------- | -------   | ----------- | --------- |
|`Authorization` | `string` | 是| tenant_access_token；注意内容不要漏了 "Bearer"| Bearer t-394890fdlkfjdajfljajdkf |

:::note
备注：请求该 URL 的应用和获取该 URL 的应用必须是同一应用。
:::


**3. Postman实例截图**
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/99b472fcf8ff06cedbafbdc5196cd2c9_soybkeJbRu.png?height=1031&lazyload=true&width=3814)
