---
document_id: '7270779605451014150'
directory_id: '7270719284443365381'
title: Selection.setSelection
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setSelection
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Selection
- Selection.setSelection
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.setSelection
---

# Selection.setSelection
设置选区信息，该方法为异步调用。
  
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
| **名称**         | **数据类型**         | **是否必填** | **描述** |
| -------------- | ---------------- | -------- | ------ |
| selectionItems | SelectionItem[] | 是        | 选区信息   |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const block = await DocMiniApp.Document.getRootBlock(docRef);
await DocMiniApp.Selection.setSelection([{ type: 'block', ref: block.ref }]);
```
