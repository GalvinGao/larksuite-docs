---
document_id: '6971043590354075653'
directory_id: '6907567266541879298'
title: offWindowResize
full_path: /uYjL24iN/uIDO3UjLygzN14iM4cTN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Window
- offWindowResize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDO3UjLygzN14iM4cTN
---

# offWindowResize(function callback)

取消监听窗口尺寸变化事件
## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | <md-version>V3.13.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |

## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


::: note
如果不传递，不会取消所有windowResize事件
:::

## 输出
无


## 代码示例

```js
const callback = function (res) {
  const size = res.size;
  const windowWidth = size.windowWidth;
  const windowHeight = size.windowHeight; 
  console.log(JSON.stringify(size))
};
 
tt.offWindowResize(callback);
```
 
