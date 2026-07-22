---
document_id: '7270779605450506246'
directory_id: '7270719284443234309'
title: View.Action.closeModal
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.closeModal
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.closeModal
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.closeModal
---

# View.Action.closeModal
关闭自定义的 modal 窗口，该方法为异步调用。

## 注意事项
该接口只能在 modal 页面中使用，其他页面使用均无效

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 模态框视图 | PC | \- |



## 输入

| **名称** | **数据类型** | **是否必填** | **描述**                          |
| ------ | -------- | -------- | ------------------------------- |
| data   | any      | 是        | 关闭 modal 窗口后给到 openModal 回调中的数据 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.View.Action.closeModal(null);
```

### 返回示例

无
