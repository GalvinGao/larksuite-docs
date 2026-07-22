---
document_id: '7073691561007808517'
directory_id: '7073450228347256837'
title: CanvasContext.setTransform
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTransform
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setTransform
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTransform
---

# CanvasContext.setTransform(number a, number b, number c, number d, number e, number f)

设置坐标转换矩阵

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| a | number | 是 |  | 水平缩放(M11) |
| b | number | 是 |  | 垂直倾斜(M12) |
| c | number | 是 |  | 水平倾斜(M21) |
| d | number | 是 |  | 垂直缩放(M22) |
| e | number | 是 |  | 水平平移(dx) |
| f | number | 是 |  | 垂直平移(dy) |


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
ctx.setTransform(1, 1, 0, 1, 0, 0);
ctx.fillRect(0, 0, 100, 100);

ctx.draw();
```
