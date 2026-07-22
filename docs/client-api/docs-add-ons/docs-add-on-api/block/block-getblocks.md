---
document_id: '7270779605450866694'
directory_id: '7270719451987738630'
title: Block.getBlocks
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.getBlocks
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.getBlocks
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.getBlocks
---

# Block.getBlocks
批量获取 Block 的快照信息，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

指定的 Block 引用数组
| **名称**    | **数据类型**                                                                                                                                           | **是否必填** | **描述**         |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | -------------- |
| blockRefs | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 指定的 Block 引用数组 |

## 输出

批量获取 Block 的快照信息，它是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)的数组
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const selectedBlocks = await DocMiniApp.Selection.getSelectedBlocks(docRef);
const blockRefs: any = [];
selectedBlocks.map((item: any) => {
  if (item.type === 'text') {
    blockRefs.push(item.ref);
  }
});
const blocks = await DocMiniApp.Block.getBlocks(blockRefs);
console.log('debug', blocks);
```

### 返回示例

```json
[
  {
    "id": 6,
    "type": "text",
    "children": [],
    "childSnapshots": [],
    "parent": 1,
    "childIndex": 1,
    "data":
    {
      "plain_text": "Asdasd",
      "text":
      {
        "elements":
          [
            {
              "text_run":
              {
                "content": "Asdasd",
                "style":
                {
                  "inline_code": false, "bold": false, "italic": false, "underline": false, "strikethrough": false
                }
              }
            }
          ]
      }
    },
    "recordId": "GmaGdSMg4o2aoKxeGXHchFL****",
    "ref":
    {
      "docRef":
      {
        "docToken": "Sj4fdE9Cro9Qg3xaHAOcv4I****"
      },
      "blockId": 6
    }
  },
  ...
]
```
