---
document_id: '7143913324617973765'
directory_id: '6907567266540699650'
title: NFCAdapter.getNfcV
full_path: /uYjL24iN/uUzM4YjL1MDO24SNzgjN/getNfcV
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.getNfcV
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:50Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUzM4YjL1MDO24SNzgjN/getNfcV
---

# NFCAdapter.getNfcV()

获取NfcV实例，实例支持NFC-V (ISO 15693)标准的读写


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.14.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.14.0+</md-version> | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |




## 输入
无


## 输出

返回值：`NfcV`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [connect](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/connect) | 连接 NfcV 类型的标签 |
| [transceive](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/transceive) | 发送数据给 NFCV 类型的标签 |
| [close](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/close) | 断开与 NFCV 标签之间的连接 |
| [getMaxTransceiveLength](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/getmaxtransceivelength) | 获取最大传输长度 |
| [setTimeout](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/settimeout) | 设置超时时间 |

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
              <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
const adapter = tt.getNFCAdapter();
const nfcV = adapter.getNfcV();
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
