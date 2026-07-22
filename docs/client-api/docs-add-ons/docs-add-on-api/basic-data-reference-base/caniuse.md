---
document_id: '7270779605450309638'
directory_id: '7270719284443561989'
title: canIUse
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/caniuse
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- canIUse
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/caniuse
---

# canIUse
检测异步 API 是否可用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 无需权限 | &nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation) | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称** | **数据类型**  | **是否必填** | **描述**         |
| ------ | --------- | -------- | -------------- |
| scopes | string[] | 是        | 异步 API 的scopes |

## 输出

异步返回查询API是否可用
  

## 示例代码

### 调用示例

```js
DocMiniApp.canIUse(['Selection', 'setSelection']).then((res) => {
    console.log('debug', res);
})
```

### 返回示例

```json
{
    "canIUse": true,
    "message": ""
}
```
