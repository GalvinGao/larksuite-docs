---
document_id: '7073693024735674373'
directory_id: '7073450228347256837'
title: CanvasContext.createCircularGradient
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createCircularGradient
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.createCircularGradient
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createCircularGradient
---

# CanvasContext.createCircularGradient(number x0, number y0, number r0)

创建圆形渐变管理对象

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| x0 | number | 是 |  | 起始点 X 坐标 |
| y0 | number | 是 |  | 起始点 Y 坐标 |
| r0 | number | 是 |  | 起始点半径 |


## 输出

返回值：
`CanvasCircularGradient`

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| addColorStop | (offset: number, color: string) => void | 添加颜色关键点 |


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
const gradient = ctx.createCircularGradient(100, 100, 100);
gradient.addColorStop(0, "white");
gradient.addColorStop(1, "green");
ctx.fillStyle = gradient;
ctx.fillRect(0, 0, 300, 200);

ctx.draw();
```
