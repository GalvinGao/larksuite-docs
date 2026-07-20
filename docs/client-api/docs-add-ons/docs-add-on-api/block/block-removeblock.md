---
document_id: '7270779605450014726'
directory_id: '7270719451987738630'
title: Block.removeBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.removeBlock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.removeBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.removeBlock
---

# Block.removeBlock
删除指定一个 Block，返回删除 Block 的快照信息，该方法为异步调用。
  
## 可用性说明
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>权限要求</md-th>
<md-th>视图可用说明</md-th>
<md-th>平台可用</md-th>
<md-th>场景</md-th></md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>可写</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

指定的 Block 引用。
| **名称**   | **数据类型**                                                                                                                                    | **是否必填** | **描述**       |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------ |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 指定的 Block 引用 |
  

## 输出

异步返回被删除 Block 的快照信息，它是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
 const DocMiniApp = new BlockitClient().initAPI();
 const docRef = await DocMiniApp.getActiveDocumentRef();
 const blockRef = DocMiniApp.getBlockRefById(docRef,4);
 const removeBlock = await DocMiniApp.Block.removeBlock(blockRef);
 console.log('debug',removeBlock);
```

### 返回示例

```json
{
  "id": 4,
  "type": "text",
  "children": [],
  "childSnapshots": [],
  "parent": 1,
  "childIndex": 5,
  "data":
  {
    "plain_text": "",
      "text":
    {
      "elements": []
    }
  },
  "recordId": "HeC8dsuseo4ockxY9UZc7JZ****"
    , "ref":
  {
    "docRef": { "docToken": "Sj4fdE9Cro9Qg3xaHAOcv4I****" }
    ,
    "blockId": 4
  }
}
```
