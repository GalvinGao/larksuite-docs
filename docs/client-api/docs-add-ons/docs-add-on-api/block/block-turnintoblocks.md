---
document_id: '7270779605451325446'
directory_id: '7270719451987738630'
title: Block.turnIntoBlocks
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.turnIntoBlocks
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.turnIntoBlocks
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.turnIntoBlocks
---

# Block.turnIntoBlocks
批量转换Block 为另一种Block，该方法为异步调用。
  
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

指定的 Block 引用以及需要转换的目标 BlockType 。
| **名称**            | **数据类型**                                                                                                                                       | **是否必填** | **描述**            |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ----------------- |
| blockRef          | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 指定的 Block 引用      |
| turnIntoBlockType | turnIntoBlockType                                                                                                                              | 是        | 需要转换的目标 BlockType |
  

## 输出

无
  

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
await DocMiniApp.Block.turnIntoBlocks(blocks, 'heading1' as TurnIntoBlockType);
```
