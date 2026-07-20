---
document_id: '7270779605451341830'
directory_id: '7270719284443283461'
title: Block.TextualBlock.getPlainText
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.getPlainText
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.getPlainText
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextualBlock.getPlainText
---

# Block.TextualBlock.getPlainText
获取文档上显示的文本数据，用于展示用。该方法是同步调用。
  
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
| **名称** | **数据类型**                                                                                                                                            | **是否必填** | **描述** |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------ |
| data   | [TextualBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textualblockdata) | 是        | 文本数据   |
  

## 输出

返回一个字符串
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefById(docRef,7);
const block = await DocMiniApp.Block.getBlock(blockRef);
const plainText =  DocMiniApp.Block.TextualBlock.getPlainText(block.data as TextBlockData);
console.log('debug',plainText);
```

### 返回示例

```
'text'
```
