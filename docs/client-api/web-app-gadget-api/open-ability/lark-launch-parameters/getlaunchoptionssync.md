---
document_id: '6965379543683563526'
directory_id: '6907567269107859458'
title: getLaunchOptionsSync
full_path: /uYjL24iN/uAzM1YjLwMTN24CMzUjN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Lark Launch Parameters
- getLaunchOptionsSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN
---

# `getLaunchOptionsSync`

::: note
可用版本：3.22+
:::

获取小程序启动时的参数。其值与 [App.onLaunch](/document/uYjL24iN/uMDNuMDNuMDN) 方法传入的参数一致，并且不会随着小程序使用而发生变化。这是一个同步方法。
::: note
注意：这种方式只能获取小程序[冷启动](/document/uYjL24iN/uMjNzUjLzYzM14yM2MTN)的页面参数。[热启动](/document/uYjL24iN/uMjNzUjLzYzM14yM2MTN)的页面参数是无法通过这种方式获取的，可以通过读取[App.onShow](/document/uYjL24iN/uYjNzUjL2YzM14iN2MTN) 或者 [Page.onLoad](/document/uYjL24iN/uYjNzUjL2YzM14iN2MTN) 方法传入的参数来获取本次[热启动](/document/uYjL24iN/uMjNzUjLzYzM14yM2MTN)的页面参数。
:::

## 代码示例
```js
var options = tt.getLaunchOptionsSync();
```

## 返回值
```json
{
	"path": "pages/index",
    "query": {
    	"foo": "bar" // 启动小程序时传入的path参数值中携带的query参数
    },
    "scene": "1000", // 场景值
    "subScene": "",
    "group_id": ""
}
```

