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
      <md-td><md-version>V3.45.0+</md-version></md-td>
      <md-td><md-version>V3.45.0+</md-version></md-td>
      <md-td><md-version>V3.45.0+</md-version></md-td>
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
          <md-td>image</md-td>
          <md-td>string</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            图片地址，支持本地地址和网络地址
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>repetition</md-td>
          <md-td>string</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            重复模式
            







**可选值**：
- `repeat` 默认值，横向和纵向重复
- `repeat-x` 横向重复
- `repeat-y` 纵向重复
- `no-repeat` 不重复图片
            
          </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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
