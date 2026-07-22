---
document_id: '6971043590353977349'
directory_id: '6907567266536505345'
title: NfcA.connect
full_path: /uYjL24iN/ucDN4YjL3QDO24yN0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NfcA
- NfcA.connect
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucDN4YjL3QDO24yN0gjN
---

# NfcA.connect(Object object)

连接 NFC-A (ISO 14443-3A) 类型的标签

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable=true fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable=true>预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           // 扫描到NFC标签后
           let nfcA = adapter.getNfcA();
           // 连接NFCA标签
           nfcA.connect({
               success(res) {
               	console.log(JSON.stringify(res))
               },
               fail(err) {
               	console.log(`NfcA.connect fail: ${JSON.stringify(res)}`)
               }
           });
      }
);
```
`success`返回对象示例：
```json
{"errMsg":"nfcConnect:ok"}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
