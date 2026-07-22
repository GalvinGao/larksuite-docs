---
document_id: '6965379543684595718'
directory_id: '6907567266541158402'
title: onAccelerometerChange
full_path: /uYjL24iN/uEzNx4SM3EjLxcTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Accelerometer
- onAccelerometerChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzNx4SM3EjLxcTM
---

# onAccelerometerChange(function callback)

监听加速度计数据。注册回调后一旦数据变化会收到结果。

::: note
调用该方法时若未打开加速度计，会调用一次 [startAccelerometer](/document/uYjL24iN/ukjNx4SO2EjL5YTM) 方法。
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-accelerometer-change/on-accelerometer-change" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| x | number | x 轴数据 |
| y | number | y 轴数据 |
| z | number | z 轴数据 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-accelerometer-change/on-accelerometer-change" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true"  appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.startAccelerometer();
tt.onAccelerometerChange(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "y": -0.5976561903953552,
    "z": -0.808624267578125,
    "x": 0.1167755201458931
}
```
