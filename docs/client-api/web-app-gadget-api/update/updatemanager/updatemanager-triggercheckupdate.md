---
document_id: '7073692582769623046'
directory_id: '7073451436034129925'
title: UpdateManager.triggerCheckUpdate
full_path: /uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/triggercheckupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- UpdateManager
- UpdateManager.triggerCheckUpdate
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/triggercheckupdate
---

# UpdateManager.triggerCheckUpdate()

主动触发更新小程序。开发者调用该方法时，首先触发[onCheckForUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate)事件；当前运行版本低于线上版本时，会强制更新小程序，若下载成功或本地已存在线上最新包，则会触发[onUpdateReady](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready)事件，否则触发[onUpdateFailed](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed)事件。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

无
## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
```js
const updateManager = tt.getUpdateManager();
updateManager.triggerCheckUpdate();
```
