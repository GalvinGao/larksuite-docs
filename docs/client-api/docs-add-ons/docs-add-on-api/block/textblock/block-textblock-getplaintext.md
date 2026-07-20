---
document_id: '7270779605450670086'
directory_id: '7270719451987722246'
title: Block.TextBlock.getPlainText
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextBlock.getPlainText
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextBlock
- Block.TextBlock.getPlainText
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextBlock.getPlainText
---

# Block.TextBlock.getPlainText
获取文档上显示的文本数据，用于展示用。该方法是同步调用。

## 注意事项
:::html
<md-alert type="warn">注意无法跟 range 对应上。</md-alert>
:::
  
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
<md-td>无需权限</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

文本数据
| **名称** | **数据类型**                                                                                                                                         | **是否必填** | **描述** |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | -------- | ------ |
| data   | [TextBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textblockdata) | 是        | 文本数据   |
  

## 输出

返回一个文档上显示的文本数据，它是一个字符串
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefById(docRef,7);
const block = await DocMiniApp.Block.getBlock(blockRef);
const plainText =  DocMiniApp.Block.TextBlock.getPlainText(block.data as TextBlockData);
console.log('debug',plainText);
```

### 返回示例

```
'text'
```
