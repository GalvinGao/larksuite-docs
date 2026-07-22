---
document_id: '6965379543683760134'
directory_id: '6907567269107597314'
title: enterChat
full_path: /uYjL24iN/ukDM04SOwQjL5ADN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- enterChat
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:12Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDM04SOwQjL5ADN
---

# enterChat(Object object)

打开指定会话

:::html
<md-alert type="tip">
小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )

[V3.8](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本后支持回调 
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| openid | string | 否 |  | 用户 [open_id](/document/home/user-identity-introduction/open-id)<br>**示例值**：ou_f096d8391d54fab99895d34519eb9e79 |
| openChatId | string | 否 |  | 会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)<br>**示例值**：oc_1965ed81fc91d3b73d68c4ca4cfc110a<br><md-alert type="tip" icon="none"><br>- Lark[V3.10](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) 和[open_id](/document/home/user-identity-introduction/open-id)都传的时候，[open_id](/document/home/user-identity-introduction/open-id)优先<br>- [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)和[open_id](/document/home/user-identity-introduction/open-id) 必须传入一个<br></md-alert> |
| needBadge | boolean | 否 | true | 是否需要展示会话页面左上角badge数<br><md-alert type="tip" icon="none"><br>- Android/iOS 端：Lark[V3.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码


```js
tt.enterChat({
    openChatId: 'oc_1965ed81fc91d3b73d68c4ca4cfc110a',
    success (res) {
        console.log(JSON.stringify(res));
    },
    fail (res) {
        console.log(`enterChat fail:${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{"errMsg":"enterChat:ok"}
``` 








