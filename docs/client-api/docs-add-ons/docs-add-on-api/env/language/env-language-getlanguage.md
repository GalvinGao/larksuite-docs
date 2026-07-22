---
document_id: '7270779605447360518'
directory_id: '7270719284443348997'
title: Env.Language.getLanguage
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.Language.getLanguage
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Env
- Language
- Env.Language.getLanguage
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.Language.getLanguage
---

# Env.Language.getLanguage
获取当前文档使用的语言，该方法为异步调用
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

异步返回当前语言，可选项：
- zh-CN
- zh-TW
- zh-HK
- en-US
- ja-JP
- fr-FR
- hi-IN
- id-ID
- it-IT
- ko-KR
- pt-BR
- ru-RU
- th-TH
- vi-VN
- de-D
- es-ES

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const language = await DocMiniApp.Env.Language.getLanguage();
console.log('debug', language);
```

### 返回示例

```
'zh-CN'
```
