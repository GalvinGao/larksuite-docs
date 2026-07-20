---
document_id: '7270779605451259910'
directory_id: '7270719451987722246'
title: Block.TextBlock.addTextStyle
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextBlock.addTextStyle
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextBlock
- Block.TextBlock.addTextStyle
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.TextBlock.addTextStyle
---

# Block.TextBlock.addTextStyle
给指定范围的文本添加样式。该方法是同步调用。
  
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

文本数据、添加的样式以及指定处理的文本范围，不指定默认全部
| **名称** | **数据类型**                                                                                                                                         | **是否必填** | **描述**            |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | -------- | ----------------- |
| data   | [TextBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textblockdata) | 是        | 文本数据              |
| style  | [TextStyle](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textrun/textstyle)                                       | 是        | 添加的样式             |
| range  | [number, number]                                                                                                                                | 否        | 指定处理的文本范围，不指定默认全部 |
  

## 输出

返回一个[TextBlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textblockdata)
  

## 示例代码

### 调用示例

```js
 const docRef = await DocMiniApp.getActiveDocumentRef();
 const blockRef = DocMiniApp.getBlockRefById(docRef,5);
 const textBlock = await DocMiniApp.Block.getBlock(blockRef);
 const newData = DocMiniApp.Block.TextBlock.addTextStyle(textBlock.data as TextBlockData,{bold: true,underline:true},[1,2]);
 console.log('debug',newData);
```

### 返回示例

```json
{
  "text":
  {
    "elements":
    [{
      "text_run": {
        "content": "测",
        "style": {
          "inline_code": false,
          "bold": false,
          "italic": false,
          "underline": false,
          "strikethrough": false
        }
      }
    },
    {
      "text_run": {
        "content": "试",
        "style": {
          "inline_code": false,
          "bold": true,
          "italic": false,
          "underline": true,
          "strikethrough": false
        }
      }
    }]
  },
  "plain_text": "测试"
}
```
