---
document_id: '6971043590354092037'
directory_id: '6907567266540699650'
title: NFCAdapter.getNfcA
full_path: /uYjL24iN/ugzM4YjL4MDO24COzgjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.getNfcA
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzM4YjL4MDO24COzgjN
---

# NFCAdapter.getNfcA()

获取NfcA实例，实例支持NFC-A (ISO 14443-3A)标准的读写


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |




## 输入
无


## 输出

返回值：`NfcA`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [connect](/document/uYjL24iN/ucDN4YjL3QDO24yN0gjN) | 连接 NfcA 类型的标签 |
| [transceive](/document/uYjL24iN/uITN4YjLyUDO24iM1gjN) | 发送数据给 NfcA 类型的标签 |
| [close](/document/uYjL24iN/uYDN4YjL2QDO24iN0gjN) | 断开与 NfcA 标签之间的连接 |
| [getAtqa](/document/uYjL24iN/ugDN4YjL4QDO24CO0gjN) | 获取 ATQA 信息 |
| [getMaxTransceiveLength](/document/uYjL24iN/ukDN4YjL5QDO24SO0gjN) | 获取最大传输长度 |
| [getSak](/document/uYjL24iN/uATN4YjLwUDO24CM1gjN) | 获取 SAK 信息 |
| [setTimeout](/document/uYjL24iN/uETN4YjLxUDO24SM1gjN) | 设置超时时间 |

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
const nfcA = adapter.getNfcA();
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
