---
document_id: '7270779605450375174'
directory_id: '7270719284443234309'
title: View.Action.showPopup
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showPopup
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.showPopup
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showPopup
---

# View.Action.showPopup
展示自定义的 Popup 窗口，窗口内容通过 app.json 的 contributes.popup[key] 指定的页面来渲染，该方法为异步调用。
  
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
<md-td>正文小组件</md-td>
<md-td>PC</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

| **名称** | **数据类型**      | **是否必填** | **描述**                                                      |
| ------ | ------------- | -------- | ----------------------------------------------------------- |
| key    | string        | 否        | 指定要渲染的 popup 页面，这个 key 需要在 app.json 的 contributes.popup 中声明 |
| style  | CSSProperties | 否        | popup 容器样式                                                  |
| data   | any           | 否        | 传递数据，Popup 页面可以通过 DocMiniApp.Bridge.getInitData 获取          |

## 输出

| **名称** | **数据类型**                      | **是否必填** | **描述**                                                                                                                               |
| ------ | ----------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| source | 'user-action' \| 'api-invoke' | 否        | popup 关闭的来源，是由 用户关闭 还是 开发者调用接口关闭<br>【用户关闭】指的是用户点击空白处自动关闭 Popup<br>【开发者调用接口关闭】指的是开发者在 popup 中调用 DocMiniApp.View.Action.hidePopup 接口 |
| data   | any                           | 是        | popup 关闭后回调的数据，指向开发者调用 hidePopup 传入的参数。如果 source 是 user-action，那么 data 会是 null。                                                      |
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const result = await DocMiniApp.View.Action.showPopup({
  style: {
    left: 0,
    top: 0,
    width: '100vw',
    height: '100vh',
    background: 'rgba(0,0,0,0.55)',
  }
});
```

### 返回示例

```json
{
    "source": "user-action",
    "data": null
}
```
