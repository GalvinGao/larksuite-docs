---
document_id: '7180270043522105350'
directory_id: '7180165099248205829'
title: offThemeChange
full_path: /uAjLw4CM/uYjL24iN/block/api/darkmode/offthemechange
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- DarkMode
- offThemeChange
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/darkmode/offthemechange
---

# offThemeChange

取消监听系统主题变化。
:::html
<md-alert type="tip">
为防止多次注册事件监听导致一次事件多次回调，建议每次调用 on 方法监听事件之前，先调用 off 方法，关闭之前的事件监听。
</md-alert>
:::

## 输入

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| callback | function | 是 | 主题变化取消监听时的回调函数 |



## 示例代码
```js
const fn = (res) => {
  console.log('取消监听');
};

tt.offThemeChange(fn);
```
