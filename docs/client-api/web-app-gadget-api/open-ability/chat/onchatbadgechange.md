---
document_id: '6965379543683497990'
directory_id: '6907567269107597314'
title: onChatBadgeChange
full_path: /uYjL24iN/uQDN2UjL0QjN14CN0YTN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- onChatBadgeChange
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDN2UjL0QjN14CN0YTN
---

# onChatBadgeChange(Object object)

监听某个群未读消息数变化

:::html
<md-alert type="tip">
注意事项：
- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )。
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
| [openChatId](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) | string | 是 |  | 获取会话信息的会话Id<br>**示例值**: oc_7dab8a3d3cdcc9da365777c7ad535d64 |
| onChange | function | 是 |  | 回调函数 |


## 输出
`onChange`回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| badge | number | 未读消息数 |


## 示例代码


```js
tt.onChatBadgeChange(
  {
    openChatId: 'oc_7dab8a3d3cdcc9da365777c7ad535d64',
    onChange: (res) => {
      console.log(res.badge)
    }
  }
)
```

回调函数返回对象示例：

```json
{
  badge: 10
}
``` 
