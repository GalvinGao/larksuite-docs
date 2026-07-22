---
document_id: '6965379543683072006'
directory_id: '6907567266540503042'
title: getSystemInfoSync
full_path: /uYjL24iN/uUjNx4SN2EjL1YTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- System Information
- getSystemInfoSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjNx4SN2EjL1YTM
---

# getSystemInfoSync()

获取系统信息



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-system-info/get-system-info" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
无
## 输出
返回值为`object`类型，其属性与异步方法的`success`返回对象参数的扩展属性相同   
请参考[getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-system-info/get-system-info" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
try {
    let result = tt.getSystemInfoSync();
    console.log(`getSystemInfoSync success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getSystemInfoSync fail: ${JSON.stringify(error)}`);
}
```

`success`返回对象示例：

```json
{
  "errMsg": "getSystemInfo:ok",
  "system": "11.4.0",
  "platform": "darwin",
  "appName": "Lark",
  "version": "5.1.0",
  "language": "zh_CN",
  "SDKVersion": "1.9.56",
  "screenWidth": 1322,
  "screenHeight": 913,
  "windowWidth": 1322,
  "windowHeight": 913,
  "pixelRatio": 2,
  "statusBarHeight": 0,
  "safeArea": {
    "left": 0,
    "right": 1322,
    "top": 0,
    "bottom": 913,
    "width": 1322,
    "height": 913
  },
  "navigationBarSafeArea": {
    "left": 0,
    "right": 1268,
    "top": 0,
    "bottom": 36,
    "width": 1268,
    "height": 36
  },
  "brand": "PC",
  "model": "PC",
  "fontSizeSetting": 12
}
``` 

## 已知问题
请参考[getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)
