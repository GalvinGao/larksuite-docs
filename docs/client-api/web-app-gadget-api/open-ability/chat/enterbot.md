---
document_id: '6965379543684415494'
directory_id: '6907567269107597314'
title: enterBot
full_path: /uYjL24iN/uAjM1EjLwITNx4CMyUTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- enterBot
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAjM1EjLwITNx4CMyUTM
---

# enterBot(Object object)

打开机器人聊天页面

:::html
<md-alert type="tip">
启用 Bot 能力即支持此 API
</md-alert>
:::

## 支持说明

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 20%;">应用能力</md-th>
<md-th style="width: 20%;">Android</md-th>
<md-th style="width: 20%;">iOS</md-th>
<md-th style="width: 20%;">PC</md-th>
<md-th style="width: 20%;">预览效果</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>小程序</md-td>
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td>/</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.enterBot({
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`enterBot fail: ${JSON.stringify(res)}`);
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "enterBot:ok"
}
```
