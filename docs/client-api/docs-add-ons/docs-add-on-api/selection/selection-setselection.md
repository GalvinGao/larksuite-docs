---
document_id: '7270779605451014150'
directory_id: '7270719284443365381'
title: Selection.setSelection
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setSelection
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Selection
- Selection.setSelection
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setSelection
---

# Selection.setSelection
设置选区信息，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

指定文档的引用以及需要选中的 Block 的 id 数组
| **名称**         | **数据类型**         | **是否必填** | **描述** |
| -------------- | ---------------- | -------- | ------ |
| selectionItems | SelectionItem[] | 是        | 选区信息   |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const block = await DocMiniApp.Document.getRootBlock(docRef);
await DocMiniApp.Selection.setSelection([{ type: 'block', ref: block.ref }]);
```
