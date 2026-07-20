---
document_id: '7073693024735903749'
directory_id: '6907567266537127937'
title: canvasToTempFilePath
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- canvasToTempFilePath
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath
---

# canvasToTempFilePath(Object object)

导出当前画布指定区域，生成图片并返回文件路径

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
          <md-td>x</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td>0</md-td>
          
          <md-td>
            导出区域 x 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>y</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td>0</md-td>
          
          <md-td>
            导出区域 y 坐标
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>width</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            导出区域宽度，默认为 Canvas 元素的宽度
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>height</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            导出区域高度，默认为 Canvas 元素的高度
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>destWidth</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            输出的图片尺寸宽度，默认为输入参数的 `width`
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>destHeight</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            输出的图片尺寸高度，默认为输入参数的 `height`
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>fileType</md-td>
          <md-td>string</md-td>
          
          <md-td>否</md-td>
          <md-td>png</md-td>
          
          <md-td>
            图片类型
            







**可选值**：
- `jpg` JPG 图片格式
- `png` PNG 图片格式
            
          </md-td>
        </md-tr>
<md-tr>
          <md-td>quality</md-td>
          <md-td>number</md-td>
          
          <md-td>否</md-td>
          <md-td>1</md-td>
          
          <md-td>
            图片质量，越大质量越高，区间为 (0, 1]
            
**示例值**: 0.3






          </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
          <md-td>tempFilePath</md-td>
          <md-td>string</md-td>
          
          <md-td>
            生成的图片临时文件路径
            







          </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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
tt.canvasToTempFilePath({
  canvasId: "demo-canvas",
  x: 0,
  y: 0,
  width: 50,
  height: 50,
  destWidth: 100,
  destHeight: 100,
  success(res) {
    console.log("TempFilePath: ", res.tempFilePath);
  },
  fail(err) {
    console.log("Error", err);
  },
  complete(res) {
    console.log("CanvasToTempFilePath Complete");
  }
});
```

返回对象示例：
```json
{
  "tempFilePath": "ttfile://user/xxxx"
}
```
