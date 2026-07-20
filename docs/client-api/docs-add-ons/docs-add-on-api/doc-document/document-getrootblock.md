---
document_id: '7270779605447327750'
directory_id: '7270719284443217925'
title: Document.getRootBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.getRootBlock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- doc-Document
- Document.getRootBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.getRootBlock
---

# Document.getRootBlock
获取指定文档的根 Block，该方法为异步调用。
  

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
<md-td>
- PC
- 移动端
</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::

## 输入

指定文档的引用
| **名称** | **数据类型**                                                                                                                                       | **是否必填** | **描述**  |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------- |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定文档的引用 |
  

## 输出

异步返回指定文档的根 Block，是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const rootblock = await DocMiniApp.Document.getRootBlock(docRef)
console.log('debug', rootblock);
```

### 返回示例

```json
{
    "childIndex": -1
    "childSnapshots": (2) [{…}, {…}]
    "children": (2) [3, 4]
    "data": {plain_text: '云文档小应用测试页面', text: {…}}
    "id": 1
    "recordId": "recordId"
    "ref": 
    "blockId" : 1
    "docRef" : {"docToken": 'docx token'}
    "type": "page"
}
```
