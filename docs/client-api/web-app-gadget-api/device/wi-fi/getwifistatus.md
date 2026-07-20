---
document_id: '6965379541105131525'
directory_id: '6907567266537832449'
title: getWifiStatus
full_path: /uYjL24iN/uYTN4QjL2UDO04iN1gDN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Wi-Fi
- getWifiStatus
document_type: GuideDocumentType
updated_at: 2021-05-23T07:10:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTN4QjL2UDO04iN1gDN
---

# `getWifiStatus`

请求获取 Wi-Fi 开关状态。
::: note
打开状态并不代表一定连接了Wifi
:::

::: note
**PC端暂不支持该API**
:::
## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，**无扩展属性**

## 输出

`success`回调对象参数的扩展属性：

名称 | 数据类型 | 描述
--|--|--|--|--
`status` | `string` | Wi-Fi 状态类型


## 代码示例

```js
tt.getWifiStatus({
	success(res){
    	console.log(res);
	},
    fail(res){
    	console.log(res)
	}
})
```

