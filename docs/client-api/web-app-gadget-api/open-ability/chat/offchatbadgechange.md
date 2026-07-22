---
document_id: '7073693024736067589'
directory_id: '6907567269107597314'
title: offChatBadgeChange
full_path: /uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- offChatBadgeChange
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:34Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange
---

# offChatBadgeChange(Object object)

取消监听某个群未读消息数变化

:::html
<md-alert type="tip">
注意事项：
- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )
- 需要读取群信息权限，接口才能调用 <md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V7.10.0+</md-version> | <md-version>V7.10.0+</md-version> | <md-version>V7.10.0+</md-version> | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| [openChatId](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) | string | 是 |  | 获取会话信息的会话Id |
| onChange | function | 否 |  | 回调函数<br><md-alert type="tip" icon="none"><br>如果不传 `onChange` 回调函数，则会取消[openChatId](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)对应的所有监听，传onChange则会取消指定监听<br></md-alert> |


## 输出
无


## 示例代码

```js
tt.offChatBadgeChange(
  {
    openChatId: 'oc_7dab8a3d3cdcc9da365777c7ad535d64',
    onChange: (res) => {
      console.log(JSON.stringify(res))
    }
  }
)
```


