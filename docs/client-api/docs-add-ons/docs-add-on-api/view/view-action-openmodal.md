---
document_id: '7270779605451538438'
directory_id: '7270719284443234309'
title: View.Action.openModal
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.openModal
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.openModal
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.openModal
---

# View.Action.openModal
展示自定义的 modal 窗口，窗口内容通过 app.json 的 `contributes.modal[key]` 指定的页面来渲染，该方法为异步调用
  
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
<md-td>\-</md-td>
</md-tr>
</md-tbody>
</md-table>
:::


## 输入

| **名称** | **数据类型** | **是否必填** | **描述**                                                                 |
| ------ | -------- | -------- | ---------------------------------------------------------------------- |
| key    | string   | 否        | 指定要渲染的 modal 页面，这个 key 需要在 app.json 的 contributes.modal 中声明。默认为 'view' |
| title  | string   | 否        | 指定 modal 的标题，如果不指定则用小应用的名称                                             |
| width  | number   | 否        | 指定 modal 的宽度，如果不指定则会使用一个固定宽度                                           |
| data   | any      | 否        | 传递数据，Modal 页面可以通过 DocMiniApp.Bridge.getInitData 获取                     |

## 输出

| **名称** | **数据类型**                      | **是否必填** | **描述**                                                                                                                                  |
| ------ | ----------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| source | 'user-action' \| 'api-invoke' | 否        | modal 关闭的来源，是由 用户关闭 还是 开发者调用接口关闭<br>【用户关闭】指的是用户点击 Modal 右上角的关闭按钮<br>【开发者调用接口关闭】指的是开发者在 modal 中调用 DocMiniApp.View.Action.closeModal 接口 |
| data   | any                           | 是        | modal 关闭后回调的数据，指向开发者调用 closeModal 传入的参数。如果 source 是 user-action，那么 data 会是 null。                                                        |
  
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const res = await DocMiniApp.View.Action.openModal({
    key: 'view',
    title: 'title', 
    width: 350,
    data: {},
});
console.log('debug', res);
```

### 返回示例

```json
{
    "source": "user-action",
    "data": null
}
```
