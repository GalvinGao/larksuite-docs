---
document_id: '7073691561008152581'
directory_id: '7073450228347256837'
title: CanvasContext.setTextBaseline
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setTextBaseline
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline
---

# CanvasContext.setTextBaseline(string align)

设置字体的对齐基线

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

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
          <md-td>align</md-td>
          <md-td>string</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            对齐方式
            







**可选值**：
- `top` 字体顶部基线
- `hanging` 字体悬停基线
- `middle` 字体中部基线
- `alphabetic` 字体默认基线
- `ideographic` 实际渲染字体基线，通常用于表意文字
- `bottom` 基线为字体底线
            
          </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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
const textBaselineList = ["top", "middle", "bottom", "normal"];

ctx.strokeStyle = "red";
ctx.moveTo(5, 75);
ctx.lineTo(295, 75);
ctx.stroke();
ctx.font = "20px sans-serif";

for (let i = 0; i < textBaselineList.length; ++i) {
  ctx.setTextBaseline(textBaselineList[i]);
  ctx.fillText(textBaselineList[i], 5 + 70 * i, 75);
}

ctx.draw();
```
