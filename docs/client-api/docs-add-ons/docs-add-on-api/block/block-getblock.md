---
document_id: '7270779605450162182'
directory_id: '7270719451987738630'
title: Block.getBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.getBlock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.getBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.getBlock
---

# Block.getBlock
获取某个 Block 的快照信息，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

指定的 Block 引用
| **名称**   | **数据类型**                                                                                                                                        | **是否必填** | **描述**       |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------ |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 指定的 Block 引用 |
  

## 输出

异步返回 指定的Block 的快照信息，它是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
 const DocMiniApp = new BlockitClient().initAPI();
 const blockRef = await DocMiniApp.getActiveBlockRef();
 const block = await DocMiniApp.Block.getBlock(blockRef);
 console.log('debug',block);
```

### 返回示例

```json
{
  "id": 3,
    "type": "isv",
      "children": [],
        "childSnapshots": [],
          "parent": 1,
            "childIndex": 0,
              "data":
  {
    "component_id": "",
      "component_type_id": "blk_63243b0cb8858002263a****",
        "data": { }
  },
  "recordId": "Oim8dqYO0o8Oi6xKwaVcE7b****",
    "ref":
  {
    "docRef":
    {
      "docToken": "Sj4fdE9Cro9Qg3xaHAOcv4I****"
    },
    "blockId": 3
  }
}
```
