---
document_id: '6971043590353928197'
directory_id: '6907567266536505345'
title: NfcA.close
full_path: /uYjL24iN/uYDN4YjL2QDO24iN0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NfcA
- NfcA.close
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDN4YjL2QDO24iN0gjN
---

# NfcA.close(Object object)

断开与NFCA标签之间的连接


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable=true fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性：

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           let nfcA = adapter.getNfcA();
        	//关闭连接
           nfcA.close({
               success(res) {
               	console.log(JSON.stringify(res))
               },
               fail(err) {
               	console.log(`NfcA.close fail: ${JSON.stringify(res)}`)
               }
           });
      }
);
```
`success`返回对象示例：
```json
{"errMsg":"nfcClose:ok"}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
