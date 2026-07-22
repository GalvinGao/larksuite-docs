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

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

展示用户卡片的配置项
  

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| userId | string | 是 | 展示的用户 id |
| placement | string | 否 | 展示卡片的位置，默认展示为 bottom，可选值：<br>- top<br>- bottom<br>- left<br>- right<br>- top-left<br>- top-right<br>- bottom-left<br>- bottom-right<br>- left-top<br>- left-bottom<br>- right-top<br>- right-bottom |
| boundingRect | object[] | 是 | 展示卡片的目标位置信息 |
| ∟x | number | 是 | 目标相对于容器（iframe）的 x 偏移值 |
| ∟y | number | 是 | 目标相对于容器（iframe）的 y 偏移值 |
| ∟width | number | 是 | 目标的宽度 |
| ∟height | number | 是 | 目标的高度 |

  

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
