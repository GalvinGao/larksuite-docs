---
document_id: '7073691561008119813'
directory_id: '6907567266541879298'
title: setWindowSize
full_path: /uYjL24iN/uEDO3UjLxgzN14SM4cTN/setwindowsize
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Window
- setWindowSize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDO3UjLxgzN14SM4cTN/setwindowsize
---

# setWindowSize(Object object)

小程序在 window 和 window-semi 模式下调整独立窗口的大小和位置

:::html
<md-alert type="tip">
无论是否有外接屏幕，x、y 都是基于Lark所在屏幕定位，当Lark和小程序不在一个屏幕时，设置 x、y 中的任意一个属性都会使小程序跳到Lark所在屏幕上
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
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td><md-version>V4.3.0+</md-version></md-td>
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
<md-td>
windowWidth
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td></md-td>
<md-td>
窗口宽度。默认值: 当前窗口宽度

**示例值**：800
</md-td>
</md-tr>
<md-tr>
<md-td>
windowHeight
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td></md-td>
<md-td>
窗口高度。默认值: 当前窗口高度

**示例值**：1000
</md-td>
</md-tr>
<md-tr>
<md-td>
x
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td></md-td>
<md-td>
窗口(以屏幕左上角为原点的) x 坐标。默认值: 当前窗口 x 坐标

**示例值**：100
</md-td>
</md-tr>
<md-tr>
<md-td>
y
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td></md-td>
<md-td>
窗口(以屏幕左上角为原点的) x 坐标。默认值: 当前窗口 y 坐标

**示例值**：100
</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.setWindowSize({
  windowWidth: 800,
  windowHeight: 1000,
  x: 100,
  y: 100,
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`setWindowSize fail: ${JSON.stringify(res)}`);
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "setWindowSize:ok"
}
```
