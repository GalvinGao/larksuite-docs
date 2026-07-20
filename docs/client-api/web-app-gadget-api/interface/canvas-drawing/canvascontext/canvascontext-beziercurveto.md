---
document_id: '7073693024735395845'
directory_id: '7073450228347256837'
title: CanvasContext.bezierCurveTo
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.bezierCurveTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo
---

# CanvasContext.bezierCurveTo(number cp1x, number cp1y, number cp2x, number cp2y, number x, number y)

添加三次贝塞尔曲线到路径中

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
          <md-td>cp1x</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            控制点1 x 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>cp1y</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            控制点1 y 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>cp2x</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            控制点2 x 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>cp2y</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            控制点2 y 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>x</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            x 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>y</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            y 坐标
            







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
ctx.beginPath();
ctx.moveTo(50, 20);
ctx.bezierCurveTo(230, 30, 150, 60, 50, 100);
ctx.stroke();

ctx.fillStyle = "blue";
// start point
ctx.fillRect(50, 20, 10, 10);
// end point
ctx.fillRect(50, 100, 10, 10);

ctx.fillStyle = "red";
// control point one
ctx.fillRect(230, 30, 10, 10);
// control point two
ctx.fillRect(150, 70, 10, 10);

ctx.draw();
```
