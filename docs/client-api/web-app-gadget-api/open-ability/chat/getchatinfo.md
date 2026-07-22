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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| openChatId | string | 是 |  | 获取会话信息的会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)<br>**示例值**：oc_1965ed81fc91d3b73d68c4ca4cfc110a |
| chatType | number | 是 |  | 会话的类型<br>**示例值**：0<br>**可选值**：<br>- `0`：单聊<br>- `1`：群聊 |
| userType | number | 否 |  | 单聊用户类型<br>**示例值**：0<br>**可选值**：<br>- `0`：用户<br>- `1`：bot<br><md-alert type="tip" icon="none"><br>chatType为0时，必须传该参数<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| i18nNames | object | 国际化会话名(可能为空)<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>zh_cn<br></md-text> | string | 中文名，可能为空 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>en_us<br></md-text> | string | 英文名，可能为空 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>ja_jp<br></md-text> | string | 日文名，可能为空 |
| name | string | 会话名称<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm> |
| avatarUrls | string[] | 会话的头像url数组，包含多种图片分辨率<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm> |
| atCount | number | 被at数量<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm><br><md-alert type="tip" icon="none"><br>Lark[V3.12.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| badge | number | 未读消息数<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm> |



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
