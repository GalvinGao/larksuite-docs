---
document_id: '7270779605447098374'
directory_id: '7270719284443283461'
title: Block.TextualBlock.clearAllHighlightTexts
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.clearAllHighlightTexts
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.clearAllHighlightTexts
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.clearAllHighlightTexts
---

# Block.TextualBlock.clearAllHighlightTexts
清除所有高亮文本，该方法为异步调用。
  
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

要清除所有高亮文本的文档
| **参数** | **类型**                                                                                                  | **必选** | **释义**       |
| ------ | ------------------------------------------------------------------------------------------------------- | ------ | ------------ |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是      | 要清除所有高亮文本的文档 |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
await DocMiniApp.Block.TextualBlock.highlightTexts(docRef);
```
