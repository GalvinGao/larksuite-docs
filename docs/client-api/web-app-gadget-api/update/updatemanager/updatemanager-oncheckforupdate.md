---
document_id: '7073692582770393094'
directory_id: '7073451436034129925'
title: UpdateManager.onCheckForUpdate
full_path: /uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- UpdateManager
- UpdateManager.onCheckForUpdate
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:03Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate
---

# UpdateManager.onCheckForUpdate(function callback)

监听向后台请求检查更新结果事件。客户端在小程序冷启动时自动检查更新，不需由开发者主动触发。线上环境在有更新内容时会触发callback回调


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 请求检查更新结果事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| hasUpdate | boolean | 是否有新的版本 |



## 示例代码

```js
const updateManager = tt.getUpdateManager();
updateManager.onCheckForUpdate(function(res) {
    console.log(`onCheckForUpdate: ${JSON.stringify(res)}`);
});
updateManager.triggerCheckUpdate();
```
回调函数返回对象示例：
```json
onCheckForUpdate: {"hasUpdate" : true}
```
