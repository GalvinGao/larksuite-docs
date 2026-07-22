---
document_id: '6971043590354042885'
directory_id: '6907567266540699650'
title: NFCAdapter.getMifareClassic
full_path: /uYjL24iN/uEDN4YjLxQDO24SM0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.getMifareClassic
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDN4YjLxQDO24SM0gjN
---

# NFCAdapter.getMifareClassic()

获取MifareClassic实例，实例支持MIFARE Classic标签的读写


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" disable=true>预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" disable=true>预览</md-preview-app> |




## 输入
无


## 输出

返回值：`MifareClassic`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [connect](/document/uYjL24iN/uQTN4YjL0UDO24CN1gjN) | 连接 MifareClassic 类型的标签 |
| [transceive](/document/uYjL24iN/ucTN4YjL3UDO24yN1gjN) | 发送数据给 MifareClassic 类型的标签 |
| [close](/document/uYjL24iN/uMTN4YjLzUDO24yM1gjN) | 断开与 MifareClassic 标签之间的连接 |
| [getMaxTransceiveLength](/document/uYjL24iN/uUTN4YjL1UDO24SN1gjN) | 获取最大传输长度 |
| [setTimeout](/document/uYjL24iN/uYTN4YjL2UDO24iN1gjN) | 设置超时时间 |

## 示例代码
:::html
<!--div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
         
    	<md-preview-app type="gadget" disable=true>预览小程序</md-preview-app>
         <md-preview-app type="webApp" disable=true>预览</md-preview-app>
  </div>
</div--> 
:::

```js
const adapter = tt.getNFCAdapter();
const mifareClassic = adapter.getMifareClassic();
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
