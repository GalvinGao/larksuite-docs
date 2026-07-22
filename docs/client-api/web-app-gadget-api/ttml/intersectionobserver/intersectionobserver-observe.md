---
document_id: '7073693024735707141'
directory_id: '7073451436033835013'
title: IntersectionObserver.observe
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- IntersectionObserver
- IntersectionObserver.observe
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe
---

# IntersectionObserver.observe(string targetSelector,function callback)

指定目标节点并开始监听相交状态变化情况


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| targetSelector | string | 是 |  | 选择器 |
| callback | function | 是 |  | 回调函数 |


## 输出

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| param | object | 回调函数接受的参数 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>intersectionRatio<br></md-text> | number | 相交比例 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>intersectionRect<br></md-text> | object | 相交区域的边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>left<br></md-text> | number | 左边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>right<br></md-text> | number | 右边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>top<br></md-text> | number | 上边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>bottom<br></md-text> | number | 下边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>boundingClientRect<br></md-text> | object | 目标边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>left<br></md-text> | number | 左边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>right<br></md-text> | number | 右边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>top<br></md-text> | number | 上边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>bottom<br></md-text> | number | 下边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>relativeRect<br></md-text> | object | 参照区域的边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>left<br></md-text> | number | 左边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>right<br></md-text> | number | 右边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>top<br></md-text> | number | 上边界 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>bottom<br></md-text> | number | 下边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>time<br></md-text> | number | 相交检测时的时间戳 |



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

