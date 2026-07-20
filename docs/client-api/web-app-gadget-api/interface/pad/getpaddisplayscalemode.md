---
document_id: '7073691561008414725'
directory_id: '7073450228347273221'
title: getPadDisplayScaleMode
full_path: /uYjL24iN/uUTOuUTOuUTO/pad/getpaddisplayscalemode
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Pad
- getPadDisplayScaleMode
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOuUTOuUTO/pad/getpaddisplayscalemode
---

# getPadDisplayScaleMode()

获取当前Pad的小程序窗口缩放状态，当前显示状态 能否进行全屏缩放的切换。


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
      <md-td><md-version>V4.10.0+</md-version></md-td>
      <md-td><md-version>V4.10.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.10.0+</md-version></md-td>
      <md-td><md-version>V4.10.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性。

## 输出

`success`返回对象参数的扩展属性：

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
            <md-td>
                displayScaleMode
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                当前Pad上的显示状态，可能的取值包括disableScale （不可全屏缩放），fullScreen（全屏，可以缩小至多栏） ，allVisible （多栏，可以放大至全屏）。 Android端仅支持 disableScale 。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码

```js
tt.getPadDisplayScaleMode({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getPadDisplayScaleMode fail: ${JSON.stringify(res)}`);
    }
});
```

返回对象示例：
```js
{
    "displayScaleMode": "allVisible",
    "errMsg": "getPadDisplayScaleMode:ok"
}
```
