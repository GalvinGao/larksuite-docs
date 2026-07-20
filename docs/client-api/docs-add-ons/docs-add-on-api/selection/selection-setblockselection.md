---
document_id: '7270779605450653702'
directory_id: '7270719284443365381'
title: Selection.setBlockSelection
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setBlockSelection
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Selection
- Selection.setBlockSelection
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setBlockSelection
---

# Selection.setBlockSelection
设置指定文档的选区为指定的 Blocks，该方法为异步调用。
  
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
<md-td>可读</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

指定文档的引用以及需要选中的 Block 的 id 数组
| **名称**   | **数据类型**                                                                                                                                       | **是否必填** | **描述**              |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------------- |
| docRef   | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定文档的引用             |
| blockIds | number[]                                                                                                                                      | 是        | 需要选中的 Block 的 id 数组 |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const rootBlock = await DocMiniApp.Document.getRootBlock(docRef);
await DocMiniApp.Selection.setBlockSelection(docRef,rootBlock.children);
```
