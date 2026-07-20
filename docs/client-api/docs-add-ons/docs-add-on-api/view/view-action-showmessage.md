---
document_id: '7270779605447147526'
directory_id: '7270719284443234309'
title: View.Action.showMessage
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showMessage
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.showMessage
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showMessage
---

# View.Action.showMessage
在文档顶部展示提示消息，消息消失后 resolve promise，该方法为异步调用。

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
<md-td>key</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>指定消息的 key，可以用于手动 hideMessage</md-td>
</md-tr>
<md-tr>
<md-td>type</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>展示的消息类型，可选值有：
- loading
- info
- success
- warning
-error</md-td>
</md-tr>
<md-tr>
<md-td>duration</md-td>
<md-td>number</md-td>
<md-td>否</md-td>
<md-td>持续多长时间（单位为 ms）后自动消失，注意设置为 0 时，表示不消失</md-td>
</md-tr>
<md-tr>
<md-td>closable</md-td>
<md-td>boolean</md-td>
<md-td>否</md-td>
<md-td>是否展示关闭按钮，默认为 false，如果 duration 设置为 0 时，该值会强制设置为 true</md-td>
</md-tr>
<md-tr>
<md-td>message</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>展示的消息内容</md-td>
</md-tr>
<md-tr>
<md-td>actionText</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>展示的消息按钮</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

onActionClick
消息按钮点击回调
| **名称**        | **数据类型** | **是否必填** | **描述**   |
| ------------- | -------- | -------- | -------- |
| onActionClick | Function | 否        | 消息按钮点击回调 |

## 输出

无
  

## 示例代码

### 调用示例A

```js
const DocMiniApp = new BlockitClient().initAPI();
await DocMiniApp.View.Action.showMessage({
    type: 'success',
    message: I18n.t('LarkCCM_DocVerse_Directory_Reset_Success'),
    actionText: '按钮'
}, () => console.log('按钮点击回调'));
```

### 返回示例

无
