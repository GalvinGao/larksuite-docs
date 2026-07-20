---
document_id: '6965379543683186694'
directory_id: '6907567266537832449'
title: getWifiList
full_path: /uYjL24iN/uUDO4UjL1gDO14SN4gTN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Wi-Fi
- getWifiList
document_type: GuideDocumentType
updated_at: 2021-05-23T07:10:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDO4UjL1gDO14SN4gTN
---

# `getWifiList`

请求获取Wifi 列表。
::: note
获取wifi列表结果会在onGetWifiList中回调返回
:::

::: note
**iOS，PC端暂不支持该API**
:::
## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，**无扩展属性**

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，**无扩展属性**


## 代码示例

```js
tt.getWifiList({
	success(res){
    	console.log(res);
	},
    fail(res){
    	console.log(res)
	}
})
```

