---
document_id: '7073691561007939589'
directory_id: '6907567269107810306'
title: getFileSystemManager
full_path: /uYjL24iN/uETOuETOuETO/tt_get_file_system_manager
breadcrumb:
- Client API
- Web app/Gadget API
- File
- getFileSystemManager
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/tt_get_file_system_manager
---

# getFileSystemManager()

获取全局唯一的文件管理器。


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
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td>**X**</md-td>
       <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app>
        </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td>**X**</md-td>
        <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::

## 目录说明

小程序文件系统分为以下 3 个部分

### 用户目录

以 `ttfile://user` 开头，开发者拥有读写权限

### 临时目录

以 `ttfile://temp` 开头，开发者拥有读权限，删权限，无写权限

### 包目录

小程序包体目录，开发者拥有读权限，无删权限，无写权限


包目录路径支持格式包括 `a/b/c`, `/a/b/c`, `./a/b/c` 且仅支持这三种格式。 例如读取包内 `app.js` 文件。

```js

const fileSystemManager = tt.getFileSystemManager();

fileSystemManager.readFile({
  filePath: "app.js",// "/app.js", "./app.js"
  encoding: "utf8",
  success(res) {
    // app.js 文件内容
    console.log(res.data);
  },
  fail(res) {
    console.error("读取失败", res.errMsg);
  },
});

```

## 输入
无


## 输出

返回值：`FileSystemManager`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td> [access(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_access) 
      </md-td> 
      <md-td>判断本地文件/目录是否存在</md-td>
    </md-tr>
    <md-tr>
      <md-td>[accessSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/access_sync)</md-td>
      <md-td>同步判断本地文件文件/目录是否存在</md-td>
    </md-tr>
    
    <md-tr>
      <md-td>[saveFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file)</md-td>
      <md-td>保存临时文件到用户目录</md-td>
    </md-tr>
    
        <md-tr>
      <md-td> [saveFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file_sync) </md-td>
      <md-td> 同步保存临时文件到用户目录 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [getFileInfo(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_file_info) </md-td>
      <md-td> 获取本地文件信息 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [getSavedFileList()](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_saved_file_list) </md-td>
      <md-td> 获取用户目录文件列表 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [removeSavedFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_remove_saved_file) </md-td>
      <md-td> 删除用户目录文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [copyFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file) </md-td>
      <md-td> 复制本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [copyFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file_sync) </md-td>
      <md-td> 同步复制本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [mkdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir) </md-td>
      <md-td> 创建本地目录 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [mkdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync) </md-td>
      <md-td> 同步创建本地目录 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [readdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir) </md-td>
      <md-td> 读取本地目录内文件列表 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [readdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir_sync) </md-td>
      <md-td> 同步读取本地目录内文件列表 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [readFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file) </md-td>
      <md-td> 读取本地文件内容 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [readFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file_sync) </md-td>
      <md-td> 同步读取本地文件内容</md-td>
    </md-tr>
    <md-tr>
      <md-td> [rename(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename) </md-td>
      <md-td> 重命名本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [renameSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename_sync)</md-td>
      <md-td>同步重命名本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [rmdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir)</md-td>
      <md-td> 删除本地目录 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [rmdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir_sync)</md-td>
      <md-td> 同步删除本地目录 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [stat(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat) </md-td>
      <md-td>获取本地文件 Stats 对象 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [statSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat_sync) </md-td>
      <md-td>同步获取本地文件 Stats 对象 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [unlink(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink) </md-td>
      <md-td> 删除本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [unlinkSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink_sync) </md-td>
      <md-td> 同步删除本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [writeFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file) </md-td>
      <md-td> 写入本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [writeFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file_sync)</md-td>
      <md-td> 同步写入本地文件 </md-td>
    </md-tr>
    <md-tr>
      <md-td> [unzip(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unzip)</md-td>
      <md-td> 解压zip文件到用户目录。 </md-td>
    </md-tr>
</md-tbody>
</md-table>
:::
## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
        <md-preview-app type="webApp"  fontSize="16" disable="true">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
try {
    let result = tt.getFileSystemManager();
    console.log(`getFileSystemManager success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getFileSystemManager fail: ${JSON.stringify(error)}`);
}
```


