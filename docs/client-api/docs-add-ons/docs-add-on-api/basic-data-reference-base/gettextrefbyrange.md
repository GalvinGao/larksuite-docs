---
document_id: '7270779605447131142'
directory_id: '7270719284443561989'
title: getTextRefByRange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/gettextrefbyrange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getTextRefByRange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/gettextrefbyrange
---

# getTextRefByRange
根据 Block 引用以及文本 range 信息，获取文本引用，该方法是同步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 无需权限 | &nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation) | - PC<br>- 移动端 | 演示模式 |



## 输入

文本所在的 Block 引用以及文本 range 信息
| **名称**   | **数据类型**                                                                                                  | **是否必填** | **描述**         |
| -------- | --------------------------------------------------------------------------------------------------------- | -------- | -------------- |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 文本所在的 Block 引用 |
| range    | Range                                                                                                     | 是        | 文本的范围          |
  

## 输出

返回文本引用，它是一个 [TextRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/TextRef)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const blockRef = DocMiniApp.getBlockRefByIndex({ docRef, blockId: 1 }, 0);
const textRef = DocMiniApp.getTextRefByRange(blockRef, [1, 3]);
console.log('debug',textRef);
```

### 返回示例

```json
{
    "docRef":{"docToken":"docx Token"},
    "parentRef":{"docRef":{"docToken":"docx Token"},
    "blockId":1},
    "index":0,
    "range":[1,3]
}
```
