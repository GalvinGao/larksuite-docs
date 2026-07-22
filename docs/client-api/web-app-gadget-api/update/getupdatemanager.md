---
document_id: '6965379543684055046'
directory_id: '6907567269107679234'
title: getUpdateManager
full_path: /uYjL24iN/uEzM04SMzQjLxMDN
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- getUpdateManager
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzM04SMzQjLxMDN
---

# getUpdateManager()

获取**全局唯一**的版本更新管理器，返回 updateManager 对象，用于管理小程序更新。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
无


## 输出

返回值：`updateManager`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [applyUpdate(Object object)](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/applyupdate) | 强制小程序重启并使用新版本。在小程序新版本下载完成后（即收到 onUpdateReady 回调）调用。 |
| [onCheckForUpdate(function callback)](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate) | 监听向后台请求检查更新结果事件。客户端在小程序冷启动时自动检查更新，不需由开发者主动触发。线上环境在有更新内容时会触发callback回调 |
| [onUpdateFailed(function callback)](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed) | 监听小程序更新失败事件。小程序有新版本，客户端主动触发下载（无需开发者触发），下载失败（可能是网络原因等）后回调 |
| [onUpdateReady(function callback)](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready) | 监听小程序有版本更新事件。客户端主动触发下载（无需开发者触发），下载成功后回调（<lebel style="color:#e45649">频率限制：>=7天且有强制更新时会回调，7天内则不回调，避免打扰用户</lebel>) |
| [triggerCheckUpdate()](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/triggercheckupdate) | 主动触发更新小程序。开发者调用该方法时，首先触发onCheckForUpdate事件；当前运行版本低于线上版本时，会强制更新小程序，若下载成功或本地已存在线上最新包，则会触发onUpdateReady事件，否则触发onUpdateFailed事件。 |

## 示例代码

```js
try {
      const updateManager = tt.getUpdateManager()
      updateManager.onCheckForUpdate(function (res) {
      	// 请求完新版本信息的回调
     	 console.log(res.hasUpdate)
      })
      updateManager.onUpdateReady(function () {
       // 新的版本已经下载好，调用 applyUpdate 应用新版本并重启
       	updateManager.applyUpdate()
      })
      updateManager.onUpdateFailed(function () {
     	// 新版本下载失败
      	console.log('download error');
      })
      // 每隔一个小时主动检查更新一次
      setInterval(() => updateManager.triggerCheckUpdate(), 60*60*1000)
	} catch (error) {
	    console.log(`getUpdateManager fail: ${JSON.stringify(error)}`);
	}
```
