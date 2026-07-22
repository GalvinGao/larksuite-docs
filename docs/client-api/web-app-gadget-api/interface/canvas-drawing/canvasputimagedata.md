---
document_id: '7073693024735789061'
directory_id: '6907567266537127937'
title: canvasPutImageData
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- canvasPutImageData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata
---

# canvasPutImageData(Object object)

更新画布像素数据

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
| data | Uint8ClampedArray | 是 |  | RGBA 像素数据 |
| x | number | 是 |  | x 坐标 |
| y | number | 是 |  | y 坐标 |
| width | number | 是 |  | 区域宽度 |
| height | number | 是 |  | 区域高度 |


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
const data = new Uint8ClampedArray(40 * 40 * 4);

for (let i = 0; i < data.length; i += 4) {
  data[i] = 0;
  data[i + 1] = 0;
  data[i + 2] = 0;
  data[i + 3] = 255;
}

tt.canvasPutImageData({
  canvasId,
  x: 80,
  y: 80,
  width: 40,
  height: 40,
  data,
  success(res) {
    console.log("success:", res);
  }
});
```
