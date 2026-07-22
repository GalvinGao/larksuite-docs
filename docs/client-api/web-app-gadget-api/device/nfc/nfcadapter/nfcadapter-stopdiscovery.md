---
document_id: '6971043590353911813'
directory_id: '6907567266540699650'
title: NFCAdapter.stopDiscovery
full_path: /uYjL24iN/uMDN4YjLzQDO24yM0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.stopDiscovery
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDN4YjLzQDO24yM0gjN
---

# NFCAdapter.stopDiscovery(Object object)

NFCAdapter.stopDiscovery(Object object) 用于关闭 NFC 标签扫描。


## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V5.25.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性。


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性。

## 示例代码

调用示例：

:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable="true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
let adapter = tt.getNFCAdapter()
adapter.stopDiscovery({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`NFCAdapter.stopDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```
`success`返回对象示例：
```json
{
    "errMsg": "nfcStopDiscovery:ok"
}
```

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见 [Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno) 或 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes).








