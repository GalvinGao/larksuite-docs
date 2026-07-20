---
document_id: '7073691561008267269'
directory_id: '7073450228347256837'
title: CanvasContext.strokeText
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeText
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.strokeText
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeText
---

# CanvasContext.strokeText(string text, number x, number y, number maxWidth)

绘制文字路径

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
          <md-td>text</md-td>
          <md-td>string</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            文字
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>x</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            y 坐标
            







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
<md-tr>
          <md-td>maxWidth</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            文字最大宽度
            







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
ctx.setFontSize(20);
ctx.strokeText("Hello Block!", 20, 20);

ctx.draw();
```
