---
document_id: '7073693024735543301'
directory_id: '7073450228347273221'
title: togglePadFullScreen
full_path: /uYjL24iN/uUTOuUTOuUTO/pad/togglepadfullscreen
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Pad
- togglePadFullScreen
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOuUTOuUTO/pad/togglepadfullscreen
---

# togglePadFullScreen()

在当前Pad小程序窗口可以全屏缩放的前提下， 进行全屏缩放状态的切换。

:::html
<md-alert type="tip">
注意事项：
- 全屏放大的状态取决于上一次全屏时候的状态，可被窗口拖拽条改变，有保留和不保留导航栏的两种形态。 **目前仅iPad侧支持该功能**。 
- 需配合 [getPadDisplayScaleMode](/document/uYjL24iN/uUTOuUTOuUTO/pad/getpaddisplayscalemode) API使用，获取当前的窗口状态，**仅 `fullScreen`和`allVisible`状态下可进行切换** 
</md-alert>
:::

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
            <md-td>
                displayScaleMode
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>''</md-td>
            <md-td>
                目标的全屏状态，fullScreen（全屏，可以缩小至多栏） ，allVisible （多栏，可以放大至全屏）。 Android端不支持

**示例值**：allVisible
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

各 callback 参数均无额外属性


## 示例代码

```js
tt.togglePadFullScreen({
    displayScaleMode: "allVisible",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`togglePadFullScreen fail: ${JSON.stringify(res)}`);
    }
});
```

返回对象示例：
```js
{
    "errMsg": "togglePadFullScreen:ok"
}
```
