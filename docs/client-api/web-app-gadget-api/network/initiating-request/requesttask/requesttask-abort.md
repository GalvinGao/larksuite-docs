---
document_id: '7073691561007972357'
directory_id: '7073451436033966085'
title: RequestTask.abort
full_path: /uYjL24iN/ugDNugDNugDN/requesttask/abort
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- initiating Request
- RequestTask
- RequestTask.abort
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:12Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/requesttask/abort
---

# RequestTask.abort()


中断请求任务



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
无



## 输出
无


## 示例代码


```js
try {
    const requestTask = tt.request({"url":"https://www.toutiao.com","data":{"noncestr":1637496519175},"header":{"content-type":"application/json"},"method":"GET","dataType":"json","responseType":"text"});
    requestTask.abort();
} catch (error) {
    console.log(`abort fail: ${JSON.stringify(error)}`);
}
```




