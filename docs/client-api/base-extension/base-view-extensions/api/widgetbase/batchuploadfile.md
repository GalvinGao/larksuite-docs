---
document_id: '7260082411118952453'
directory_id: '7258197168736698374'
title: batchUploadFile
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/batchuploadfile
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- batchUploadFile
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/batchuploadfile
---

# base.batchUploadFile
批量上传文件接口，按序返回当前文件列表对应的 file token 列表。token可以用于设置附件类型的字段值。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入

```
batchUploadFile(file)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| file | [File对象](https://developer.mozilla.org/zh-CN/docs/Web/API/File)数组或者[FileList](https://developer.mozilla.org/zh-CN/docs/Web/API/FileList)。 | 是 | [File对象](https://developer.mozilla.org/zh-CN/docs/Web/API/File)数组或者[FileList](https://developer.mozilla.org/zh-CN/docs/Web/API/FileList)。 |



## 输出
Promise字符串数组。
## 示例代码

```js
  const file = new File(['Hello, World!'], 'hello.txt', { type: 'text/plain' });
  
  const tokens =await bitable.base.batchUploadFile([file]); // 拿到的token可以用于设置附件字段

  console.log(tokens) // ['BcdqbMmW4ohD7ExUq9rcGtuVn8e']
```

