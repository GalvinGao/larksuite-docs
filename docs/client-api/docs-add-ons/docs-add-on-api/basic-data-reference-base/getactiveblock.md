---
document_id: '7270779605447245830'
directory_id: '7270719284443561989'
title: getActiveBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactiveblock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getActiveBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactiveblock
---

# getActiveBlock
获取当前小组件 Block（文档块）的快照信息，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | - 正文小组件<br>- 全屏视图<br>- 模态框视图<br>- 弹窗视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数
  

## 输出

异步返回当前所在 Block（文档块）的快照信息，它是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.getActiveBlock()
  .then((blockSnpapshot) => {
    console.log('debug', blockSnpapshot);
  })
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
