---
document_id: '7073693024735756293'
directory_id: '7073450228347256837'
title: CanvasContext.draw
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.draw
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw
---

# CanvasContext.draw(boolean reverse, function callback)

将所有的操作绘制到 Canvas 中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| reverse | boolean | 否 | false | 是否保留之前的 Canvas 绘制结果 |
| callback | function | 否 |  | 绘制完成之后触发回调函数 |


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
ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 100);
ctx.draw();
ctx.fillRect(50, 50, 150, 100);
ctx.draw(true, () => {
  console.log("Last Draw End.");
});
```
