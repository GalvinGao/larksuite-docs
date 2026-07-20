---
document_id: '7270779605447311366'
directory_id: '7270719284443217925'
title: Document.getPageMeta
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.getPageMeta
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- doc-Document
- Document.getPageMeta
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Document.getPageMeta
---

# Document.getPageMeta
获取指定文档的 meta 信息，该方法为异步调用。

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
</md-tr>
</md-tbody>
</md-table>
:::


## 输入

指定文档的引用
| **名称** | **数据类型**                                                                                                                                       | **是否必填** | **描述**  |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------- |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定文档的引用 |

## 输出

异步返回指定文档的 meta 信息，它是一个 [PageMeta](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/PageMeta)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const meta = await DocMiniApp.Document.getPageMeta(docRef)
console.log('debug', meta);   
```

### 返回示例

```json
{
  "comments_count":0,
  "comments_count_today":0,
  "create_timestamp":1673426766,
  "like_count":0,
  "like_count_today":0,
  "pv":1,
  "pv_today":1,
  "uv":1,
  "uv_today":1,
  "owner_user":
  {
    "id":"710148447583340****",
    "cn_name":"名字",
    "en_name":"zi MING",
    "avatar_url":"https://xxxavatar_urlxxxx"
  "char_count":22,
  "word_count":12
}
```
