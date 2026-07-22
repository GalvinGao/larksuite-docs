---
document_id: '6965379567070494725'
directory_id: '6907567266540765186'
title: createIntersectionObserver
full_path: /uYjL24iN/ucTNwEjL3UDMx4yN1ATM
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- createIntersectionObserver
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucTNwEjL3UDMx4yN1ATM
---

# createIntersectionObserver(object instance,object option)

创建并返回一个 IntersectionObserver 对象实例，以观察目标元素与另一参照元素（或视口）的重叠状态。在页面或自定义组件中，也可以使用 `this.createIntersectionObserver([options])`来代替


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| instance | object | 是 |  | 页面或组件实例 |
| option | object | 否 |  | 创建选项 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>thresholds<br></md-text> | number[] | 否 | [0] | 一个数值数组，包含所有阈值 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>initialRatio<br></md-text> | number | 否 | 0 | 初始的相交比例，如果调用时检测到的相交比例与这个值不相等且达到阈值，则会触发一次监听器的回调函数。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>observeAll<br></md-text> | boolean | 否 | false | 是否同时观测多个目标节点（而非一个），如果设为 true ，observe 的 targetSelector 将选中多个节点（注意：同时选中过多节点将影响渲染性能） |



## 输出
返回值：`intersectionObserver`，该对象的方法列表参见下表：


:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [IntersectionObserver.observe](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe) | 指定目标节点并开始监听相交状态变化情况 |
| [IntersectionObserver.relativeTo](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativeto) | 使用选择器指定一个节点，作为参照区域之一 |
| [IntersectionObserver.relativeToViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport) | 指定页面显示区域作为参照区域之一 |
| [IntersectionObserver.disconnect](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/disconnect) | 停止监听，回调函数将不再触发 |

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
//也可以使用 this.createIntersectionObserver({selectAll}) 来创建
tt.createIntersectionObserver(this, {
    selectAll: true
})
.relativeTo('.container')
.observe('.ball', res => {
    res.intersectionRect  // 相交区域
    res.intersectionRect.left  // 相交区域的左边界坐标
    res.intersectionRect.top  // 相交区域的上边界坐标
    res.intersectionRect.width  // 相交区域的宽度
    res.intersectionRect.height  // 相交区域的高度
});
```


