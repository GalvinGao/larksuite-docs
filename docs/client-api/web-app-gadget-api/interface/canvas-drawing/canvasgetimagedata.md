---
document_id: '7073692582769131526'
directory_id: '6907567266537127937'
title: canvasGetImageData
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvasgetimagedata
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- canvasGetImageData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasgetimagedata
---

# canvasGetImageData(Object object)

获取画布像素数据

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.45.0+</md-version> | <md-version>V3.45.0+</md-version> | <md-version>V3.45.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| canvasId | string | 是 |  | 需要操纵的 Canvas 组件上的 canvas-id 的值 |
| x | number | 是 |  | x 坐标 |
| y | number | 是 |  | y 坐标 |
| width | number | 是 |  | 区域宽度 |
| height | number | 是 |  | 区域高度 |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| width | number | 返回像素数据的宽度 |
| height | number | 返回像素数据的高度 |
| data | Uint8ClampedArray | RGBA 像素数据 |


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
tt.canvasGetImageData({
  canvasId,
  x: 0,
  y: 0,
  width: 40,
  height: 40,
  success(res) {
    console.log("CanvasGetImageData Response Width:", res.width);
    console.log("CanvasGetImageData Response Height:", res.height);
    console.log("CanvasGetImageData Response data test:", res.data instanceof Uint8ClampedArray); // true
    console.log("CanvasGetImageData Response data length:", res.data.length); // 40 * 40 * 4
    console.log("CanvasGetImageData Response data:", res.data);
  },
  fail(err) {
    console.log("Error", err);
  },
  complete() {
    console.log("GetImageData Complete");
  }
});
```

返回对象示例：
```json
{
  "width": 40,
  "height": 40,
  "data": "<Uint8ClampedArray Instance>"
}
```
