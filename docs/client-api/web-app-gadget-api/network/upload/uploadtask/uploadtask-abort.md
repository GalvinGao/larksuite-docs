---
document_id: '7073692582769950726'
directory_id: '7073451436033949701'
title: UploadTask.abort
full_path: /uYjL24iN/ugDNugDNugDN/uploadtask/abort
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- Upload
- UploadTask
- UploadTask.abort
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:18Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/uploadtask/abort
---

# UploadTask.abort()

`uplaodFile`调用结果在通过回调传递的同时会返回一个`uploadTask`对象，可以通过该对象的`abort`方法中断请求任务。

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

无

## 输出

无

## 示例代码

```js
  const uploadTask = tt.uploadFile({
    "url": "https://cloudapi.bytedance.net/faas/services/tt594x/invoke/imgupload",
    "filePath": filePath,
    "name": "test.jpeg"
  })
  uploadTask.onProgressUpdate(function(res) {
    console.log("uploading")
    console.log(res)
    uploadTask.abort()
  })
```
