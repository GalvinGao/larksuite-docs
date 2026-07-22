---
document_id: '6965379541103968261'
directory_id: '6907567266541371394'
title: onCompassChange
full_path: /uYjL24iN/uQzNx4CN3EjL0cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Compass
- onCompassChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzNx4CN3EjL0cTM
---

# onCompassChange(function callback)

监听罗盘数据变化事件，频率：5 次/秒，接口调用后会自动开始监听，可使用 [stopCompass](/document/uYjL24iN/uMzNx4yM3EjLzcTM) 停止监听。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| direction | number | 面对的方向度数 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.onCompassChange(function(res) {
    console.log(JSON.stringify(res));
});
```

返回值示例：
```json
{
    "direction": 86.3572986955703
}
```
