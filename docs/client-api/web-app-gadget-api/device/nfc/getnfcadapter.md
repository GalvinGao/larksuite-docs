---
document_id: '6965379541104476165'
directory_id: '6907567266541273090'
title: getNFCAdapter
full_path: /uYjL24iN/ukzM4YjL5MDO24SOzgjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- getNFCAdapter
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:42Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzM4YjL5MDO24SOzgjN
---

# getNFCAdapter()

获取客户端NFC适配器


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+<md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |




## 输入
无


## 输出

返回值：`NFCAdapter`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [getNfcA](/document/uYjL24iN/ugzM4YjL4MDO24COzgjN) | 获取NfcA实例，实例支持NFC-A (ISO 14443-3A)标准的读写 |
| [getNfcV](/document/uYjL24iN/uUzM4YjL1MDO24SNzgjN/getNfcV) | 获取NfcV实例，实例支持NFC-V (ISO 15693)标准的读写<br><md-alert type="tip" icon="none"><br>Lark[V5.14](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [getMifareClassic](/document/uYjL24iN/uEDN4YjLxQDO24SM0gjN) | 获取 MifareClassic 实例，实例支持 MIFARE Classic 标签的读写 |
| [startDiscovery](/document/uYjL24iN/uIDN4YjLyQDO24iM0gjN) | 开始扫描 NFC 标签 |
| [stopDiscovery](/document/uYjL24iN/uMDN4YjLzQDO24yM0gjN) | 关闭 NFC 标签扫描 |
| [onDiscovered](/document/uYjL24iN/uUDN4YjL1QDO24SN0gjN) | 监听 NFC Tag |
| [offDiscovered](/document/uYjL24iN/uQDN4YjL0QDO24CN0gjN) | 取消监听 NFC Tag |

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
const adapter = tt.getNFCAdapter()
```
    
## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
