---
document_id: '7270779605447163910'
directory_id: '7270719284443594757'
title: Bridge.getInitData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Bridge.getInitData
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Bridge
- Bridge.getInitData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Bridge.getInitData
---

# Bridge.getInitData
获取创建应用时透传的初始化数据，该方法为异步调用。

## 主要事项
目前只有 View.Action.openModal 和 View.Action.showPopup 接口支持传递

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

Promise<any>
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const initData = await DocMiniApp.Bridge.getInitData();
console.log('debug', initData);
```

### 返回示例
```json
  {}
```
  
> 非Modal/Popup会返回undefined
