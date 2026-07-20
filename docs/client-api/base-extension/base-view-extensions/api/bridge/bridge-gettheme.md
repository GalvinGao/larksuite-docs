---
document_id: '7260082411118264325'
directory_id: '7258197168736632838'
title: bridge.getTheme
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_gettheme
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.getTheme
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_gettheme
---

# bridge.getTheme
获取当前主题。





## 输出
Promise 字符串，主题色，值为以下值之一：
```js
export enum ThemeModeType {
  LIGHT = 'LIGHT',
  DARK = 'DARK',
}
```
## 示例代码
### 调用示例

```js
const res = await bitable.bridge.getTheme();
```


### 返回示例
res:
```js
'LIGHT'
```
