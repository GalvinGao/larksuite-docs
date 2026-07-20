---
document_id: '7270779605450571782'
directory_id: '7270719284443561989'
title: getActiveBlockRef
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactiveblockref
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getActiveBlockRef
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactiveblockref
---

# getActiveBlockRef
返回当前正文小组件所在 Block（文档块）的引用，该方法为异步调用。
  

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
<md-td>&nbsp;正文小组件<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation)</md-td>
<md-td>
- PC
- 移动端
</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::

## 输入

无需传入参数。
  

## 输出

异步返回当前正文小组件所在 Block（文档块）的引用，它是一个[BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.getActiveBlockRef()
  .then((blockRef) => {
    console.log('debug', blockRef);
  })
```

### 返回示例

```json
{
  "docRef":
  {
    "docToken": "docx token"
  },
  "blockId": 5
}
```
