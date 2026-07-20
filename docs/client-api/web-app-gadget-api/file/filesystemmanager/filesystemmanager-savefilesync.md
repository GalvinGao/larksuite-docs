---
document_id: '7073691561008463877'
directory_id: '7073451436034048005'
title: FileSystemManager.saveFileSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.saveFileSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:48Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file_sync
---

# FileSystemManager.saveFileSync

保存临时文件到用户目录

::: note
该 API 会把临时文件**移动**到永久目录，所以在调用成功后原文件路径将访问失败。
:::

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
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" 
path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td>**X**</md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" 
fontSize="14">预览</md-preview-app></md-td>
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
      <md-td>tempFilePath</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        临时文件路径
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>filePath</md-td>
      <md-td>string</md-td>
      <md-td>否</md-td>
      <md-td></md-td>
      <md-td>
        用户文件路径
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>savedFilePath</md-td>
      <md-td>string</md-td>
      <md-td>
       	保存后文件路径
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::

## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    // 获取图片, chooseImage 获取的文件在临时文件目录内
    const tempFilePaths = res.tempFilePaths;
    if (tempFilePaths[0]) {
      // 保存到用户目录
      const savedFilePath = fileSystemManager.saveFileSync(tempFilePaths[0]);
      console.log(`文件已经从 ${tempFilePaths[0]} 移动到 ${savedFilePath}`);
    }
  },
});
```

## 已知问题

- iOS和Android 用户目录存储的大小限制为 200M。


