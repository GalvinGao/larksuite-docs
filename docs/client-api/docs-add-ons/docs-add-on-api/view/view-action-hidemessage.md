---
document_id: '7270779605447262214'
directory_id: '7270719284443234309'
title: View.Action.hideMessage
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.hideMessage
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.hideMessage
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.hideMessage
---

# View.Action.hideMessage
将显示的消息手动隐藏，触发 showMessage 的 promise resolve。这里的 key 对应 showMessage 的 `options.key`，该方法为异步调用。

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称** | **数据类型** | **是否必填** | **描述**                     |
| ------ | -------- | -------- | -------------------------- |
| key    | string   | 是        | showMessage 的`options.key` |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
await DocMiniApp.View.Action.hideMessage('messageKey');
```

### 返回示例

无
