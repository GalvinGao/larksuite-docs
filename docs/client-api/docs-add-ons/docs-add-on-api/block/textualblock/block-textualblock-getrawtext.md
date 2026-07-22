---
document_id: '7270779605450883078'
directory_id: '7270719284443283461'
title: Block.TextualBlock.getRawText
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.getRawText
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.getRawText
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.getRawText
---

# Block.TextualBlock.getRawText
获取原始的文本数据，可以跟 range 对应上。该方法是同步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 无需权限 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

文本数据
| **名称** | **数据类型**                                                                                                                                            | **是否必填** | **描述** |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------ |
| data   | [TextualBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textualblockdata) | 是        | 文本数据   |
  

## 输出

返回一个字符串
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefById(docRef,7);
const block = await DocMiniApp.Block.getBlock(blockRef);
const rawText =  DocMiniApp.Block.TextualBlock.getRawText(block.data as TextBlockData);
```

### 返回示例

```
'text'
```
