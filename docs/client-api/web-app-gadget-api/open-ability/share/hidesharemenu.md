---
document_id: '6965379541104738309'
directory_id: '7073448001569914886'
title: hideShareMenu
full_path: /uYjL24iN/ukjN24SO2YjL5YjN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Share
- hideShareMenu
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjN24SO2YjL5YjN
---

# hideShareMenu(Object object)

隐藏当前页面的分享按钮

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
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share_menu/share_menu" fontSize="14">预览</md-preview-app>
</md-td>
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
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share_menu/share_menu" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::
```js
tt.hideShareMenu({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`hideShareMenu fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "hideShareMenu:ok"
}
```


