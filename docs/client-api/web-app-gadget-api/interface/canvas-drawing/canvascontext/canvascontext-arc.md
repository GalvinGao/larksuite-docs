---
document_id: '7073692582769655814'
directory_id: '7073450228347256837'
title: CanvasContext.arc
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arc
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.arc
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arc
---

# CanvasContext.arc(number x, number y, number radius, number startAngle, number endAngle, string counterclockwise)

添加圆弧到当前路径中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| x | number | 是 |  | 圆心 x 坐标 |
| y | number | 是 |  | 圆心 y 坐标 |
| radius | number | 是 |  | 半径 |
| startAngle | number | 是 |  | 圆开始弧度 |
| endAngle | number | 是 |  | 圆结束弧度 |
| counterclockwise | string | 否 | false | 是否是逆时针计算 |


## 输出

无

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
    <md-preview-app type="gadget" disable="true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::



```javascript
ctx.arc(100, 100, 60, 0, 1.5 * Math.PI, false);
ctx.fill();

ctx.draw();
```
