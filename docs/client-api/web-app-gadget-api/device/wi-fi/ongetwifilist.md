---
document_id: '6965379543683366918'
directory_id: '6907567266537832449'
title: onGetWifiList
full_path: /uYjL24iN/uYDO4UjL2gDO14iN4gTN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Wi-Fi
- onGetWifiList
document_type: GuideDocumentType
updated_at: 2021-05-23T07:10:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDO4UjL2gDO14iN4gTN
---

# `onGetWifiList`

监听获取到 Wi-Fi 列表数据事件



## 输入

function callback
获取到 Wi-Fi 列表数据事件的回调函数

回调参数
名称 | 数据类型 | 描述
--|--|--|--|--
`wifiList` | `wifiInfo` |Wi-Fi 列表数据


wifiInfo数据
名称 | 数据类型 | 描述
--|--|--|--|--
`SSID` | `string` | Wi-Fi 的 SSID
`BSSID` | `string` | Wi-Fi 的 BSSID
`secure	` | `boolean` | Wi-Fi 是否安全
`signalStrength` | `number` | Wi-Fi 信号强度

## 代码示例

```js
var callback = function(res){
	console.log(res)
}
tt.onGetWifiList(callback)
tt.offGetWifiList(callback)
```
