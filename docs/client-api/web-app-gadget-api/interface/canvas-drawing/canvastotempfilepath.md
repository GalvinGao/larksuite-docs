---
document_id: '7073693024735903749'
directory_id: '6907567266537127937'
title: canvasToTempFilePath
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- canvasToTempFilePath
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath
---

# canvasToTempFilePath(Object object)

导出当前画布指定区域，生成图片并返回文件路径

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| canvasId | string | 是 |  | 需要操纵的 Canvas 组件上的 canvas-id 的值 |
| x | number | 否 | 0 | 导出区域 x 坐标 |
| y | number | 否 | 0 | 导出区域 y 坐标 |
| width | number | 否 |  | 导出区域宽度，默认为 Canvas 元素的宽度 |
| height | number | 否 |  | 导出区域高度，默认为 Canvas 元素的高度 |
| destWidth | number | 否 |  | 输出的图片尺寸宽度，默认为输入参数的 `width` |
| destHeight | number | 否 |  | 输出的图片尺寸高度，默认为输入参数的 `height` |
| fileType | string | 否 | png | 图片类型<br>**可选值**：<br>- `jpg` JPG 图片格式<br>- `png` PNG 图片格式 |
| quality | number | 否 | 1 | 图片质量，越大质量越高，区间为 (0, 1]<br>**示例值**: 0.3 |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tempFilePath | string | 生成的图片临时文件路径 |


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
tt.canvasToTempFilePath({
  canvasId: "demo-canvas",
  x: 0,
  y: 0,
  width: 50,
  height: 50,
  destWidth: 100,
  destHeight: 100,
  success(res) {
    console.log("TempFilePath: ", res.tempFilePath);
  },
  fail(err) {
    console.log("Error", err);
  },
  complete(res) {
    console.log("CanvasToTempFilePath Complete");
  }
});
```

返回对象示例：
```json
{
  "tempFilePath": "ttfile://user/xxxx"
}
```
