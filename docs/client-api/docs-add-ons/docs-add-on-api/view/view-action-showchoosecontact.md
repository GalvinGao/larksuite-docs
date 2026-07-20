---
document_id: '7270779605451358214'
directory_id: '7270719284443234309'
title: View.Action.showChooseContact
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showChooseContact
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.showChooseContact
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showChooseContact
---

# View.Action.showChooseContact
展示联系人选择器，用户选择之后将结果返回，该方法为异步调用。
  
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

联系人选择器的配置选项
| **名称**         | **数据类型** | **是否必填** | **描述**       |
| -------------- | -------- | -------- | ------------ |
| title          | string   | 否        | 联系人选择器的标题    |
| description    | string   | 否        | 联系人选择器的描述    |
| multiSelect    | boolean  | 否        | 是否支持选择多个联系人  |
| maxSelectedNum | number   | 否        | 支持最多选择多少个联系人 |
  

## 输出

联系人选择器的返回结果
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>名称</md-th>
<md-th>数据类型</md-th>
<md-th>是否必填</md-th>
<md-th>描述</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>联系人选择器结果类型，可选值：
  - confirm：确认
  - cancel：取消
</md-td>
</md-tr>
<md-tr>
<md-td>chats</md-td>
<md-td>object[]</md-td>
<md-td>是</md-td>
<md-td>已选择的联系人</md-td>
</md-tr>
<md-tr>
<md-td>∟id</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>联系人的 id</md-td>
</md-tr>
<md-tr>
<md-td>∟name</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>联系人的名称</md-td>
</md-tr>
<md-tr>
<md-td>∟avatar</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>联系人的头像</md-td>
</md-tr>
</md-tbody>
</md-table>
:::
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const res = await DocMiniApp.View.Action.showChooseContact({
    title: '选择联系人',
    multiSelect: true,
    maxSelectedNum: 10,
});
console.log('debug', res);
```

### 返回示例

```json
{
    "type": "confirm",
    "contacts": [
        {
            "id": "联系人的 id",
            "name": "联系人的名称"
        }
    ]
}
```
