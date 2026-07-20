---
document_id: '6965379543684366342'
directory_id: '6907567269107597314'
title: getChatInfo
full_path: /uYjL24iN/uEDN2UjLxQjN14SM0YTN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- getChatInfo
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN
---

# getChatInfo(Object object)

获取某个会话的信息

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
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
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
                openChatId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                获取会话信息的会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**示例值**：oc_1965ed81fc91d3b73d68c4ca4cfc110a
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                chatType
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                会话的类型

**示例值**：0

**可选值**：
- `0`：单聊
- `1`：群聊
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                userType
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                单聊用户类型

**示例值**：0

**可选值**：
- `0`：用户
- `1`：bot
              
<md-alert type="tip" icon="none">
chatType为0时，必须传该参数
</md-alert>                 
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
                i18nNames
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                国际化会话名(可能为空)

**字段权限要求**：
<md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
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
                    zh_cn
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                中文名，可能为空
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
                    en_us
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                英文名，可能为空
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
                    ja_jp
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                日文名，可能为空
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                name
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                会话名称

**字段权限要求**：
<md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                avatarUrls
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                会话的头像url数组，包含多种图片分辨率
              
**字段权限要求**：
<md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                atCount
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                被at数量
                            
**字段权限要求**：
<md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
              
<md-alert type="tip" icon="none">
Lark[V3.12.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                badge
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                未读消息数
                            
**字段权限要求**：
<md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码

```js
tt.getChatInfo(
  {
    openChatId: 'oc_1965ed81fc91d3b73d68c4ca4cfc110a',
    chatType:  0,
    userType:  0,
    success (res) {
        console.log(JSON.stringify(res));
    },
    fail (res) {
        console.log(`getChatInfo fail:${JSON.stringify(res)}`);
    }
  }
)
```

`success`返回对象示例：

```json
{
  "atCount": 0,
  "avatarUrls": [
    "https://s3-imfile.feishucdn.com/static-resource/v1/f8836d60-07cb-4fd4-92ce-01e54952f79g~?image_size=72x72&cut_type=&quality=&format=png&sticker_format=.webp",
    "https://s3-imfile.feishucdn.com/static-resource/v1/f8836d60-07cb-4fd4-92ce-01e54952f79g~?image_size=240x240&cut_type=&quality=&format=png&sticker_format=.webp",
    "https://s3-imfile.feishucdn.com/static-resource/v1/f8836d60-07cb-4fd4-92ce-01e54952f79g~?image_size=noop&cut_type=&quality=&format=png&sticker_format=.webp",
    "https://s3-imfile.feishucdn.com/static-resource/v1/f8836d60-07cb-4fd4-92ce-01e54952f79g~?image_size=640x640&cut_type=&quality=&format=png&sticker_format=.webp"
  ],
  "badge": 0,
  "i18nNames": {
    "en_us": "Lark会议",
    "zh_cn": "Lark会议"
  },
  "name": "Lark会议",
  "errMsg": "getChatInfo:ok"
}
``` 

*(关键词：chatid, chat_id, chat id, 群id)*
