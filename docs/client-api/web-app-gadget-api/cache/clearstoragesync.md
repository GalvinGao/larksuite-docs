---
document_id: '6965379541104099333'
directory_id: '6907567266536308737'
title: clearStorageSync
full_path: /uYjL24iN/uYTOx4iN5EjL2kTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- clearStorageSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:39Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTOx4iN5EjL2kTM
---

# clearStorageSync()

清理**全部**本地缓存数据。


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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
无

## 输出
无

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
      <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::


```js
try {
    tt.clearStorageSync();
} catch (error) {
    console.log(`clearStorageSync fail: ${JSON.stringify(error)}`);
}
```
