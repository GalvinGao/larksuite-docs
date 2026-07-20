---
document_id: '7270779700748877829'
directory_id: '7270719284443561989'
title: getActiveWikiToken
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactivewikitoken
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getActiveWikiToken
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getactivewikitoken
---

# getActiveWikiToken
获取当前文档的 wiki token，该方法为异步调用。
  
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
<md-td>&nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation)
</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

无需传入参数。
  

## 输出

异步返回当前文档的 wiki token，它是一个string类型
  

## 示例代码

### 调用示例

```js	
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.getActiveWikiToken()
  .then((wikiToken) => {
    console.log('debug', wikiToken);
  })
```

### 返回示例

```json
'wikiToken'
```
