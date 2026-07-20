---
document_id: '7073691561008381957'
directory_id: '7073451436034048005'
title: FileSystemManager.access
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_access
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.access
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:50Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_access
---

# FileSystemManager.access(Object object)

判断文件/目录是否存在。

## 支持说明

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 20%;">应用能力</md-th>
<md-th style="width: 20%;">Android</md-th>
<md-th style="width: 20%;">iOS</md-th>
<md-th style="width: 20%;">PC</md-th>
<md-th style="width: 20%;">预览效果</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>小程序</md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
</md-tbody>
</md-table>
:::
## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>path</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        要判断是否存在的文件/目录路径（包目录/临时目录/用户目录）

      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();
const path = `ttfile://temp/some_path`; // 判断的地址

fileSystemManager.access({
  path,
  success(_res) {
    console.log(`${path} 地址存在`);
  },
  fail(res) {
    console.log(`${path} 地址不存在或其他错误`, res.errMsg);
  },
});
```

