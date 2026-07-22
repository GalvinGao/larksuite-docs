---
document_id: '7073693024735461381'
directory_id: '7073450228347256837'
title: CanvasContext.drawImage
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-drawImage
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.drawImage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-drawImage
---

# CanvasContext.drawImage(string image, number sx, number sy, number sw, number sh, number dx, number dy, number dw, number dh)

绘制图像到画布

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| image | string | 否 |  | 需要绘制的图片路径<br><md-alert type="tip" icon="none"><br>- 支持本地图片（调用其他API得到的路径，如[chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM)、[downloadFile](/document/uYjL24iN/ucDMx4yNwEjL3ATM)等）<br>- 支持网络图片，可以直接使用<br>- 不允许跨域网络图片直接使用，可以通过[downloadFile](/document/uYjL24iN/ucDMx4yNwEjL3ATM)下载到本地后使用<br></md-alert> |
| sx | number | 否 |  | 需要绘制到画布中的，image的矩形（裁剪）选择框的左上角 x 坐标 |
| sy | number | 否 |  | 需要绘制到画布中的，image的矩形（裁剪）选择框的左上角 y 坐标 |
| sw | number | 否 |  | 需要绘制到画布中的，image的矩形（裁剪）选择框的宽度 |
| sh | number | 否 |  | 需要绘制到画布中的，image的矩形（裁剪）选择框的高度 |
| dx | number | 否 |  | image的左上角在目标 canvas 上 x 轴的位置 |
| dy | number | 否 |  | image的左上角在目标 canvas 上 y 轴的位置 |
| dw | number | 否 |  | 在目标画布上绘制image的宽度，允许对绘制的image进行缩放 |
| dh | number | 否 |  | 在目标画布上绘制image的高度，允许对绘制的image进行缩放 |


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
ctx.drawImage(
  "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4788e761425c502c0c1302a95ceb920f.png",
  0,
  0,
  150,
  100
);
ctx.draw();
```
