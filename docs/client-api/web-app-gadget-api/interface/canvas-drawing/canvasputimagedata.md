---
document_id: '7073693024735789061'
directory_id: '6907567266537127937'
title: canvasPutImageData
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- canvasPutImageData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata
---

# canvasPutImageData(Object object)

更新画布像素数据

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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
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
          <md-td>canvasId</md-td>
          <md-td>string</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            需要操纵的 Canvas 组件上的 canvas-id 的值
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>data</md-td>
          <md-td>Uint8ClampedArray</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            RGBA 像素数据
            







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
<md-tr>
          <md-td>width</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            区域宽度
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>height</md-td>
          <md-td>number</md-td>
          
          <md-td>是</md-td>
          <md-td></md-td>
          
          <md-td>
            区域高度
            







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
const data = new Uint8ClampedArray(40 * 40 * 4);

for (let i = 0; i < data.length; i += 4) {
  data[i] = 0;
  data[i + 1] = 0;
  data[i + 2] = 0;
  data[i + 3] = 255;
}

tt.canvasPutImageData({
  canvasId,
  x: 80,
  y: 80,
  width: 40,
  height: 40,
  data,
  success(res) {
    console.log("success:", res);
  }
});
```
