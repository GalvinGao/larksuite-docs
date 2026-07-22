---
document_id: '7073692582770229254'
directory_id: '7073450228347256837'
title: CanvasContext.createPattern
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createPattern
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.createPattern
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createPattern
---

# CanvasContext.createPattern(string image, string repetition)

创建径向渐变管理对象

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.45.0+</md-version> | <md-version>V3.45.0+</md-version> | <md-version>V3.45.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| image | string | 是 |  | 图片地址，支持本地地址和网络地址 |
| repetition | string | 是 |  | 重复模式<br>**可选值**：<br>- `repeat` 默认值，横向和纵向重复<br>- `repeat-x` 横向重复<br>- `repeat-y` 纵向重复<br>- `no-repeat` 不重复图片 |


## 输出

返回值：
`CanvasPattern`

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
const pattern = ctx.createPattern("https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4788e761425c502c0c1302a95ceb920f.png", "repeat-x");
ctx.fillStyle = pattern;
ctx.fillRect(0, 0, 300, 150);
ctx.draw();
```
