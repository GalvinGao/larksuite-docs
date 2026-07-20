---
document_id: '7270779700748812293'
directory_id: '7270719284443234309'
title: View.Action.showUserProfile
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showUserProfile
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- View
- View.Action.showUserProfile
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/View.Action.showUserProfile
---

# View.Action.showUserProfile
展示用户卡片，该方法为异步调用。
  
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

展示用户卡片的配置项
  
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>名称</md-th>
<md-th>数据类型</md-th>
<md-th>是否必填</md-th>
<md-th>描述</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>userId</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>展示的用户 id</md-td>
</md-tr>
<md-tr>
<md-td>placement</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>展示卡片的位置，默认展示为 bottom，可选值：
- top
- bottom
- left
- right
- top-left
- top-right
- bottom-left
- bottom-right
- left-top
- left-bottom
- right-top
- right-bottom
  </md-td>
</md-tr>
<md-tr>
<md-td>boundingRect</md-td>
<md-td>object[]</md-td>
<md-td>是</md-td>
<md-td>展示卡片的目标位置信息</md-td>
</md-tr>
<md-tr>
<md-td>∟x</md-td>
<md-td>number</md-td>
<md-td>是</md-td>
<md-td>目标相对于容器（iframe）的 x 偏移值</md-td>
</md-tr>
<md-tr>
<md-td>∟y</md-td>
<md-td>number</md-td>
<md-td>是</md-td>
<md-td>目标相对于容器（iframe）的 y 偏移值</md-td>
</md-tr>
<md-tr>
<md-td>∟width</md-td>
<md-td>number</md-td>
<md-td>是</md-td>
<md-td>目标的宽度</md-td>
</md-tr>
<md-tr>
<md-td>∟height</md-td>
<md-td>number</md-td>
<md-td>是</md-td>
<md-td>目标的高度</md-td>
</md-tr>
</md-tbody>
</md-table>
:::
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.View.Action.showUserProfile({
  userId: 'user id',
  placement: 'right-bottom',
  boundingRect: {
    x: 0,
    y: 0,
    width: 100,
    height: 180,
  }
});
```

### 返回示例

无
