---
document_id: '7073693024735838213'
directory_id: '7073451436034129925'
title: UpdateManager.onUpdateReady
full_path: /uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- UpdateManager
- UpdateManager.onUpdateReady
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:03Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready
---

# UpdateManager.onUpdateReady(function callback)

监听小程序有版本更新事件。客户端主动触发下载（无需开发者触发），下载成功后回调


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 小程序有版本更新事件的回调函数 |


## 输出
无


## 示例代码
```js
const updateManager = tt.getUpdateManager();
updateManager.onUpdateReady(function(res) {
    console.log(`onUpdateReady:${JSON.stringify(res)}`);
});
updateManager.triggerCheckUpdate();
```
回调函数返回对象示例：
```json
onUpdateReady:{}
```


