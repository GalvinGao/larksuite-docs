---
document_id: '6965379543684464646'
directory_id: '6907567266537832449'
title: offGetWifiList
full_path: /uYjL24iN/ucDO4UjL3gDO14yN4gTN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Wi-Fi
- offGetWifiList
document_type: GuideDocumentType
updated_at: 2021-05-23T07:10:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucDO4UjL3gDO14yN4gTN
---

# `offGetWifiList`

取消监听获取到 Wi-Fi 列表数据事件


## 输入

function callback
获取到 Wi-Fi 列表数据事件的回调函数

## 实例代码
```js
var callback = function(res){
	console.log(res)
}
tt.onGetWifiList(callback)
tt.offGetWifiList(callback)
```
