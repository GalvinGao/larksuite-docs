---
document_id: '6965379541104721925'
directory_id: '6907567266541879298'
title: onWindowResize
full_path: /uYjL24iN/uADO3UjLwgzN14CM4cTN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Window
- onWindowResize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADO3UjLwgzN14CM4cTN
---

# onWindowResize(function callback)

监听窗口尺寸变化事件

:::html
<md-alert type="tip">
使用同一回调函数多次调用，会注册多次该事件，回调会被执行多次。
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | <md-version>V3.13.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| size | object | 窗口大小 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>windowWidth<br></md-text> | number | 变化后的窗口宽度，单位px |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>windowHeight<br></md-text> | number | 变化后的窗口高度，单位px |


## 示例代码


```js
tt.onWindowResize(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "size": {
        "windowHeight": 768,
        "windowWidth": 507
    }
}
```
