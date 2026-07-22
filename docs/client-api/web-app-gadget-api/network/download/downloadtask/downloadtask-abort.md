---
document_id: '7073691561008037893'
directory_id: '7073451436033933317'
title: DownloadTask.abort
full_path: /uYjL24iN/ugDNugDNugDN/downloadfile/abort
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- Download
- "\bDownloadTask"
- DownloadTask.abort
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/downloadfile/abort
---

# DownloadTask.abort()

`downloadFile`的调用结果在通过回调传递的同时会返回一个`downloadTask`对象，可以通过该对象的`abort`方法中断请求任务。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

无

## 输出

无

## 示例代码

```js
const downloadTask2 = tt.downloadFile({"url":"https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fimg.jj20.com%2Fup%2Fallimg%2Ftp05%2F19100120461512E-0-lp.jpg&refer=http%3A%2F%2Fimg.jj20.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=jpeg?sec=1640079653&t=22aafb14cb145c11fc833022d61507c5"});
downloadTask2.abort(function(res) {
    console.log(JSON.stringify(res));
});
```
