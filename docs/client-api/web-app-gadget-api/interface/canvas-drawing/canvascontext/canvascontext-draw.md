---
document_id: '7073693024735756293'
directory_id: '7073450228347256837'
title: CanvasContext.draw
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.draw
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw
---

# CanvasContext.draw(boolean reverse, function callback)

将所有的操作绘制到 Canvas 中

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
          <md-td>reverse</md-td>
          <md-td>boolean</md-td>
          
          <md-td>否</md-td>
          <md-td>false</md-td>
          
          <md-td>
            是否保留之前的 Canvas 绘制结果
            







          </md-td>
        </md-tr>
<md-tr>
          <md-td>callback</md-td>
          <md-td>function</md-td>
          
          <md-td>否</md-td>
          <md-td></md-td>
          
          <md-td>
            绘制完成之后触发回调函数
            







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
ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 100);
ctx.draw();
ctx.fillRect(50, 50, 150, 100);
ctx.draw(true, () => {
  console.log("Last Draw End.");
});
```
