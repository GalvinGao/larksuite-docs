---
document_id: '7270779605447098374'
directory_id: '7270719284443283461'
title: Block.TextualBlock.clearAllHighlightTexts
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.clearAllHighlightTexts
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.clearAllHighlightTexts
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.clearAllHighlightTexts
---

# Block.TextualBlock.clearAllHighlightTexts
清除所有高亮文本，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

要清除所有高亮文本的文档
| **参数** | **类型**                                                                                                  | **必选** | **释义**       |
| ------ | ------------------------------------------------------------------------------------------------------- | ------ | ------------ |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是      | 要清除所有高亮文本的文档 |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
await DocMiniApp.Block.TextualBlock.highlightTexts(docRef);
```
