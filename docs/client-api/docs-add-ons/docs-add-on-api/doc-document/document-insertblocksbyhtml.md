---
document_id: '7270779605450145798'
directory_id: '7270719284443217925'
title: Document.insertBlocksByHTML
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.insertBlocksByHTML
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- doc-Document
- Document.insertBlocksByHTML
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.insertBlocksByHTML
---

# Document.insertBlocksByHTML
将 HTML 转化为 Blocks 并插入到文档中，该方法为异步调用。
  
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

指定文档的引用以及设置的文档标题。
| **名称**   | **数据类型**                                                                                                                                    | **是否必填** | **描述**               |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------- | -------- | -------------------- |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 需要插入 html 的 block 引用 |
| html     | string                                                                                                                                      | 是        | 需要插入的 html           |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const blockRef = await DocMiniApp.getActiveBlockRef();
await DocMiniApp.Document.insertBlocksByHTML(blockRef,'<span>我是一段html<span>')
```
