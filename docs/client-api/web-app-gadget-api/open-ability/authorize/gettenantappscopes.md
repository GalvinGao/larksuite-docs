---
document_id: '7407276518294323206'
directory_id: '6907567266537635841'
title: getTenantAppScopes
full_path: /uYjL24iN/uczMx4yNzEjL3MTM/gettenantappscopes
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Authorize
- getTenantAppScopes
document_type: GuideDocumentType
updated_at: 2024-08-30T02:32:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczMx4yNzEjL3MTM/gettenantappscopes
---

# getTenantAppScopes(Object object)

应用拥有所需权限后，才能调用Lark接口获取相关信息。为了让数据能更好地被保护，开放平台对权限的等级进行拆分，该方法用于查询租户下该应用授权状态。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.43.0+</md-version> | <md-version>V3.43.0+</md-version> | <md-version>V3.43.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| scopes | scope[] | 权限列表 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 权限名称 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>status<br></md-text> | number | 授权状态<br>**可选值**：<br>- `1`：已授权<br>- `2`：未授权 |


## 示例代码

```js
tt.getTenantAppScopes({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`openSetting fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
  "scopes": [
    { "name": "im:message.p2p_msg:readonly", "status": 1 },
    { "name": "im:message:send_as_bot", "status": 1 },
    { "name": "contact:user.identity:readonly", "status": 1 },
    { "name": "docs:docs:operate_as_user", "status": 1 },
    { "name": "im:message.group_at_msg:readonly", "status": 1 },
    { "name": "contact:user.base:readonly", "status": 1 },
    { "name": "im:chat.group_info:readonly", "status": 1 }
  ]
}
```
