---
document_id: '7260082411118804997'
directory_id: '7258197168736632838'
title: bridge.getLanguage
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getlanguage
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.getLanguage
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getlanguage
---

# bridge.getLanguage
获取当前文档语言。





## 输出
Promise 字符串，值为以下语言之一：
```js
export type Language =
  | 'zh'
  | 'zh-TW'
  | 'zh-HK'
  | 'en'
  | 'ja'
  | 'fr'
  | 'hi'
  | 'id'
  | 'it'
  | 'ko'
  | 'pt'
  | 'ru'
  | 'th'
  | 'vi'
  | 'de'
  | 'es';
```
## 示例代码
### 调用示例

```js
const res = await bitable.bridge.getLanguage();
```


### 返回示例
res:
```js
'zh'
```
