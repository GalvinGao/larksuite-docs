---
document_id: '7073692582769115142'
directory_id: '7073450228347256837'
title: CanvasContext.setLineDash
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setLineDash
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash
---

# CanvasContext.setLineDash(number[] segments, number offset)

设置间断线

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
          <md-td>segments</md-td>
          <md-td>number[]</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            间断线的分块
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>offset</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            间断线起点偏移值
            







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
let height = 15;

const drawDashedLine = (pattern) => {
  ctx.beginPath();
  ctx.setLineDash(pattern);
  ctx.moveTo(0, height);
  ctx.lineTo(300, height);
  ctx.stroke();
  height += 20;
};

drawDashedLine([]);
drawDashedLine([1, 1]);
drawDashedLine([10, 10]);
drawDashedLine([20, 5]);
drawDashedLine([15, 3, 3, 3]);
drawDashedLine([20, 3, 3, 3, 3, 3, 3, 3]);
drawDashedLine([12, 3, 3]);  // Equals [12, 3, 3, 12, 3, 3]

ctx.draw();
```
