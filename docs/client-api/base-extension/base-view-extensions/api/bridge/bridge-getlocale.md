---
document_id: '7260082411118313477'
directory_id: '7258197168736632838'
title: bridge.getLocale
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getlocale
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.getLocale
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getlocale
---

# bridge.getLocale
获取当前文档语言配置。





## 输出
Promise 字符串，值为以下语言之一：
```js
export type Locale =
  | 'zh-CN'
  | 'zh-TW'
  | 'zh-HK'
  | 'en-US'
  | 'ja-JP'
  | 'fr-FR'
  | 'hi-IN'
  | 'id-ID'
  | 'it-IT'
  | 'ko-KR'
  | 'pt-BR'
  | 'ru-RU'
  | 'th-TH'
  | 'vi-VN'
  | 'de-DE'
  | 'es-ES';
```
## 示例代码
### 调用示例

```js
const res = await bitable.bridge.getLocale();
```


### 返回示例
res:
```js
'zh-CN'
```
