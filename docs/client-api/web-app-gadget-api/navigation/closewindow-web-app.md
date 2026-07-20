---
document_id: '7346095924105560069'
directory_id: '6907567266537734145'
title: closeWindow（网页应用）
full_path: /uYjL24iN/uYTOuYTOuYTO/closewindow
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- closeWindow (Web app)
document_type: GuideDocumentType
updated_at: 2024-03-14T07:04:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTOuYTOuYTO/closewindow
---

# closeWindow(Object object)

关闭当前窗口

:::html
<md-alert type="tip">
无需 [网页应用鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)，即可调用此API，但仍需要保证在 [window.h5sdk.ready](/document/uYjL24iN/uITO4IjLykDOy4iM5gjM) 的回调函数触发后调用

</md-alert>

<md-alert type="tip">

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
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
	</md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.14.0+</md-version></md-td>
      <md-td><md-version>V5.14.0+</md-version></md-td>
      <md-td><md-version>V5.14.0+</md-version></md-td><md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
    </md-tr>
    
  </md-tbody>
</md-table>
:::







## 示例代码
```js
window.h5sdk.ready(() => { // ready方法不需要每次都调用
  tt.closeWindow({
      fail(res) {
        console.log(`closeWindow fail: ${JSON.stringify(res)}`);
      }
  });
});
```
`fail`返回对象示例：
```json
{
    "errMsg": "closeWindow:fail unknown error"
}
```

