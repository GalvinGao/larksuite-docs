---
document_id: '7180269945544392710'
directory_id: '7180165099250991109'
title: hideProfile
full_path: /uAjLw4CM/uYjL24iN/block/api/user/hideprofile
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- User
- hideProfile
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:22Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/user/hideprofile
---

# hideProfile



关闭用户信息卡片。

:::note
该 API 仅云文档 PC 环境支持。
:::

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)。

## 输出

各 callback 返回对象参数均无额外扩展属性。

## 示例代码

### 调用示例

```js
tt.hideProfile({
  success (res) {
    console.log('hideProfile 调用成功', res.errMsg);
  },
  fail (res) {
    console.log('hideProfile 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('hideProfile 调用结束', res.errMsg);
  } 
});
```

### 返回示例

```json
{
  "errMsg": "hideProfile:ok"
}
```
