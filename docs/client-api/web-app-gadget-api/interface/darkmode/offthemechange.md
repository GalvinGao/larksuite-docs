---
document_id: '7073691561008250885'
directory_id: '7073450228347240453'
title: offThemeChange
full_path: /uYjL24iN/uUTOuUTOuUTO/darkmode/offthemechange
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- DarkMode
- offThemeChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOuUTOuUTO/darkmode/offthemechange
---

# offThemeChange(function callback)

取消监听系统主题变化的事件
## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.3.0+</md-version> | <md-version>V5.3.0+</md-version> | <md-version>V5.3.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |

## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


::: note
如果不传递，不会取消所有 themeChange 事件
:::

## 输出
无


## 代码示例

```js
const fn = ({ theme }) => {
  console.log('onThemeChange', theme);
};
tt.onThemeChange(fn);
// 在某个时机取消监听
tt.offThemeChange(fn);
```
 
