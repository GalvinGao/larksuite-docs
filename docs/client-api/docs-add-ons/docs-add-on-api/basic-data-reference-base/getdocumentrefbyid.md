---
document_id: '7270779700749008901'
directory_id: '7270719284443561989'
title: getDocumentRefById
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getdocumentrefbyid
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getDocumentRefById
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getdocumentrefbyid
---

# getDocumentRefById
根据文档 token 获取指定文档的引用，该方法是同步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 无需权限 | &nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation) | - PC<br>- 移动端 | 演示模式 |



## 输入

文档 token（从文档URL直接获取）。
| **名称**   | **数据类型** | **是否必填** | **描述**   |
| -------- | -------- | -------- | -------- |
| docToken | string   | 是        | 文档 token |
  

## 输出

返回一个指定文档的引用，它是 [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const activeDocumentRef = DocMiniApp.getDocumentRefById('docx token');
console.log('debug',activeDocumentRef);
```

### 返回示例

```json
{"docToken":"docx token"}
```
