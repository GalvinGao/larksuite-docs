---
document_id: '7073691561008513029'
directory_id: '7073450228347256837'
title: CanvasContext.createLinearGradient
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createLinearGradient
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.createLinearGradient
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createLinearGradient
---

# CanvasContext.createLinearGradient(number x0, number y0, number x1, number y1)

创建线性渐变对象

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
          <md-td>x0</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            起始点 X 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>y0</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            起始点 Y 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>x1</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            终止点的 X 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>y1</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            终止点的 Y 坐标
            







          </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

返回值：
`CanvasLinearGradient`

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
// Create linear gradient
const gradient = ctx.createLinearGradient(0, 0, 200, 0);
gradient.addColorStop(0, "red");
gradient.addColorStop(1, "white");

// Fill with gradient
ctx.setFillStyle(gradient);
ctx.fillRect(10, 10, 150, 80);

ctx.draw();
```
