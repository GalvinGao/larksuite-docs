---
document_id: '7330180313525796870'
directory_id: '6907567266540797954'
title: closeBluetoothAdapter
full_path: /uYjL24iN/uYDOxYjL2gTM24iN4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- closeBluetoothAdapter
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDOxYjL2gTM24iN4EjN
---

# closeBluetoothAdapter(Object object)

closeBluetoothAdapter(Object object) 该接口用于关闭蓝牙模块。

:::note
调用该接口将断开所有已建立的蓝牙连接，并释放系统资源。建议在使用蓝牙流程后，与 `tt.openBluetoothAdapter` 接口成对调用。
:::

## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25.0+</md-version> | <md-version>V3.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.closeBluetoothAdapter({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`closeBluetoothAdapter fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "closeBluetoothAdapter:ok"
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "closeBluetoothAdapter:fail not init",
    "errCode": 10000
}
```


## 错误码

`fail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。

**errCode 错误码**

通用错误码可参见[蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)。

**errno 错误码**

关于 Errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
