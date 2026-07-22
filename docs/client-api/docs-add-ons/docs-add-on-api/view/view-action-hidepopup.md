---
document_id: '7270779700748943365'
directory_id: '7270719284443234309'
title: View.Action.hidePopup
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.hidePopup
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.hidePopup
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.hidePopup
---

# View.Action.hidePopup
关闭自定义的 popup 窗口，该方法为异步调用。

## 主要事项
注意该接口只能在 popup 页面中使用，其他页面使用均无效

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 弹窗视图 | PC | 演示模式 |



## 输入

| **名称** | **数据类型** | **是否必填** | **描述**                          |
| ------ | -------- | -------- | ------------------------------- |
| data   | any      | 是        | 关闭 popup 窗口后给到 showPopup 回调中的数据 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.View.Action.hidePopup(null);
```

### 返回示例

无
