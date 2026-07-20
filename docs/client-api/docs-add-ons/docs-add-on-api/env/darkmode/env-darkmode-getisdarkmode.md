---
document_id: '7270779605450686470'
directory_id: '7270719284443643909'
title: Env.DarkMode.getIsDarkMode
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.DarkMode.getIsDarkMode
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Env
- DarkMode
- Env.DarkMode.getIsDarkMode
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.DarkMode.getIsDarkMode
---

# Env.DarkMode.getIsDarkMode
获取当前是否暗黑模式，该方法为异步调用。
  
## 可用性说明
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>权限要求</md-th>
<md-th>视图可用说明</md-th>
<md-th>平台可用</md-th>
<md-th>场景</md-th></md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>可读</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

无需传入参数。
  

## 输出

返回一个布尔值，true为暗黑模式，false为非暗黑模式

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const isDarkModePromise = await docMiniApp.Env.DarkMode.getIsDarkMode();
console.log('debug', isDarkModePromise);
```

### 返回示例

```
false
```
