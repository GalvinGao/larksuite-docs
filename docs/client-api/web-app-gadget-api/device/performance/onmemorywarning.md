---
document_id: '7073691561008300037'
directory_id: '7073451436034097157'
title: onMemoryWarning
full_path: /uYjL24iN/uQTOuQTOuQTO/performance/onmemorywarning
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- performance
- onMemoryWarning
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/performance/onmemorywarning
---

# onMemoryWarning(function callback)

监听内存不足的告警事件。当手机内存占用过高时，触发回调函数。该事件不会杀掉小程序, 建议开发者可以在接受到告警后释放不必要的资源。 在Android平台下有告警等级划分。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.4.0+</md-version> | <md-version>V5.4.0+</md-version> | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | **/** |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| level | number | 对应系统内存告警等级宏（Level）定义，数值越高，告警等级越高。<br>**可选值**：<br>- `5`：TRIM_MEMORY_RUNNING_MODERATE<br>- `10`：TRIM_MEMORY_RUNNING_LOW<br>- `15`：TRIM_MEMORY_RUNNING_CRITICAL<br><md-alert type="tip" icon="none"><br>仅 Android 端返回该字段<br></md-alert> |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">

  <div style="display: flex">
  </div>
</div> 
:::

```js
tt.onMemoryWarning(function () {
  console.log("onMemoryWarning");
});
```

回调函数返回对象示例（仅Android）：

```json
{"level":"5"}
