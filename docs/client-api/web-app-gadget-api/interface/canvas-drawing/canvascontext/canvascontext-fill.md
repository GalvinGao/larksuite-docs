---
document_id: '7073692582769852422'
directory_id: '7073450228347256837'
title: CanvasContext.fill
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.fill
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill
---

# CanvasContext.fill()

填充当前路径

## 支持说明
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
    </md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::


## 输入

无

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
// begin path
ctx.rect(10, 10, 100, 30);
ctx.fillStyle = "yellow";
ctx.fill();

ctx.beginPath();
ctx.rect(10, 40, 100, 30);

// only fill this rect, not in current path
ctx.setFillStyle("blue");
ctx.fillRect(10, 70, 100, 30);

ctx.rect(10, 100, 100, 30);

// it will fill current path
ctx.setFillStyle("red");
ctx.fill();
ctx.draw();
```
