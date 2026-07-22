---
document_id: '7073692582769410054'
directory_id: '7073451436033949701'
title: UploadTask.onProgressUpdate
full_path: /uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- Upload
- UploadTask
- UploadTask.onProgressUpdate
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:18Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate
---

# UploadTask.onProgressUpdate(function callback)

`upoadFile`的调用结果在通过回调传递的同时会返回一个`uploadTask`对象，通过`onProgressUpdate`方法监听上传进度。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

无

## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| progress | number | 上传进度 |
| totalBytesSent | number | 已经上传的数据长度，单位 byte |
| totalBytesExpectedToSend | number | 预期需要上传的数据总长度，单位 byte |



## 示例代码

```js
  const uploadTask = tt.uploadFile({
    "url": "https://cloudapi.bytedance.net/faas/services/tt594x/invoke/imgupload",
    "filePath": filePath,
    "name": "test.jpeg"
  })
  uploadTask.onProgressUpdate(function(res) {
    console.log("uploading")
    if (res.progress == 100) {
	  // upload complete
      console.log(res)
    }
  })
```

回调函数返回对象示例：

```json
{
    "totalBytesExpectedToSend": 118561,
    "totalBytesSent": 118561,
    "progress": 100
}
``` 


