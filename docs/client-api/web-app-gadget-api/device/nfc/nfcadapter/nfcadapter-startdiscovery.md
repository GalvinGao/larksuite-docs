---
document_id: '6971043590353944581'
directory_id: '6907567266540699650'
title: NFCAdapter.startDiscovery
full_path: /uYjL24iN/uIDN4YjLyQDO24iM0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.startDiscovery
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:58Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDN4YjLyQDO24iM0gjN
---

# NFCAdapter.startDiscovery(Object object)

开始扫描NFC标签


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


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
let adapter = tt.getNFCAdapter()
adapter.startDiscovery({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`NFCAdapter.startDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```
`success`返回对象示例：
```json
{
    "nfcSuccess": "start to listener nfc card",
    "errMsg": "nfcStartDiscovery:ok"
}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
