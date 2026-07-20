---
document_id: '7270779605451440134'
directory_id: '7270719284443234309'
title: View.Action.showChooseChat
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showChooseChat
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.showChooseChat
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showChooseChat
---

# View.Action.showChooseChat
展示聊天会话选择器，用户选择之后将结果返回，该方法为异步调用。
  
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

聊天会话选择器配置选项
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
<md-td>title</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>聊天会话选择器的标题</md-td>
</md-tr>
<md-tr>
<md-td>description</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>聊天会话选择器的描述</md-td>
</md-tr>
<md-tr>
<md-td>multiSelect</md-td>
<md-td>boolean</md-td>
<md-td>否</md-td>
<md-td>是否支持选择多个聊天会话</md-td>
</md-tr>
<md-tr>
<md-td>maxSelectedNum</md-td>
<md-td>number</md-td>
<md-td>否</md-td>
<md-td>支持最多选择多少个聊天会话</md-td>
</md-tr>
<md-tr>
<md-td>selectType</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>可以选择的聊天会话的类型，可选择：
  - all_chats：全部聊天会话
- group_chats：多人聊天
- p2p_chats：私聊
- bot：机器人
- without_cross_tenant_chats：非跨租户聊天
  </md-td>
</md-tr>
</md-tbody>
</md-table>
:::

  

## 输出

聊天会话选择器的返回结果
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
<md-td>聊天会话选择器结果类型，可选值：
- confirm：确认
- cancel：取消</md-td>
</md-tr>
<md-tr>
<md-td>chats</md-td>
<md-td>object[]</md-td>
<md-td>是</md-td>
<md-td>已选择的聊天会话</md-td>
</md-tr>
<md-tr>
<md-td>∟id</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>聊天会话的 id</md-td>
</md-tr>
<md-tr>
<md-td>∟name</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>聊天会话的名称</md-td>
</md-tr>
<md-tr>
<md-td>∟chatType</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>聊天会话的类型，可选值：
- P2P：单聊
- GROUP：群聊
- TOPIC_GROUP：话题群
- UNKNOWN：未知</md-td>
</md-tr>
</md-tbody>
</md-table>
:::
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const res = await DocMiniApp.View.Action.showChooseChat({
    title: '选择要发送的会话',
    selectType: 'group_chats',
    multiSelect: true,
    maxSelectedNum: 10,
});
console.log('debug', res);
```

### 返回示例

```json
{
    "type": "confirm",
    "chats": [
        {
            "id": "会话id",
            "name": "会话名称",
            "chatType": "GROUP"
        }
    ]
}
```
