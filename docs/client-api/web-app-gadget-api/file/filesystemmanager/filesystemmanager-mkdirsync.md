---
document_id: '7073693024735346693'
directory_id: '7073451436034048005'
title: FileSystemManager.mkdirSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.mkdirSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync
---

# FileSystemManager.mkdirSync

创建本地目录。

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
<md-td>**X**</md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td>**X**</md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输入

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
      <md-td>dirPath</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        创建的目录路径, 必须 ttfile://user 开头
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>recursive</md-td>
      <md-td>boolean</md-td>
      <md-td>否</md-td>
      <md-td>false</md-td>
      <md-td>
        是否递归创建该目录的上级目录后再创建该目录。
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

try {
  fileSystemManager.mkdirSync("ttfile://user/some/path", true);
  console.log("调用成功");
} catch (err) {
  console.log("调用失败", err);
}
```

