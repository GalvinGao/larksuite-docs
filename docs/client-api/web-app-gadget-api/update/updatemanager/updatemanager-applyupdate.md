---
document_id: '7073691561008480261'
directory_id: '7073451436034129925'
title: UpdateManager.applyUpdate
full_path: /uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/applyupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- UpdateManager
- UpdateManager.applyUpdate
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:03Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/applyupdate
---

# UpdateManager.applyUpdate(Object object)

强制小程序重启并使用新版本。在小程序新版本下载完成后（即收到 [onUpdateReady](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready) 回调）调用。

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
      <md-td> <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> 
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

```js
const updateManager = tt.getUpdateManager();
updateManager.applyUpdate({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`applyUpdate fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "applyUpdate:ok"
}
```


