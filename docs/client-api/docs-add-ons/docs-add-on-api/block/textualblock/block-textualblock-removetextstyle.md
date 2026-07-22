---
document_id: '7270779605450489862'
directory_id: '7270719284443283461'
title: Block.TextualBlock.removeTextStyle
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.removeTextStyle
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.removeTextStyle
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.removeTextStyle
---

# Block.TextualBlock.removeTextStyle
给指定范围的文本删除样式。该方法是同步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 无需权限 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

文本数据以及指定处理的文本范围，不指定默认全部
| **名称** | **数据类型**                                                                                                                                            | **是否必填** | **描述**            |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ----------------- |
| data   | [TextualBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textualblockdata) | 是        | 文本数据              |
| range  | [number, number]                                                                                                                                   | 否        | 指定处理的文本范围，不指定默认全部 |
  

## 输出

返回删除样式后的BlockData，它是一个TextBlockData 对象
  

## 示例代码

### 调用示例

```js
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefById(docRef,2);
const block = await DocMiniApp.Block.getBlock(blockRef);
const newData = DocMiniApp.Block.TextualBlock.removeTextStyle(block.data as TextBlockData,[1,2]);
const updatedBlock = await DocMiniApp.Block.updateBlock(blockRef,newData);
```

### 返回示例

```json
{
  "plain_text": "测试",
    "text": {
    "elements":
    [
      {
        "text_run": {
          "content": "测试",
          "style": { "inline_code": false, "bold": false, "italic": false, "underline": false, "strikethrough": false }
        }
      }
    ]
  }
}
```
