---
document_id: '7073693024735559685'
directory_id: '7073451436034048005'
title: FileSystemManager.stat
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.stat
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat
---

# FileSystemManager.stat(Object object)

获取本地文件 Stats 对象。


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
      <md-td><md-version>V5.23.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" 
path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"
fontSize="14">预览</md-preview-app> </md-td>
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
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                path
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                本地文件路径

**示例值**：ttfile://temp
              

            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

`success`返回对象的扩展属性与方法：
:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
:::html

<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                stat
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                Stats 对象
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    mode
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                文件的类型和存取的权限，对应 POSIX stat.st_mode
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    size
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                文件大小，单位：B，对应 POSIX stat.st_size
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    lastAccessedTime
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                文件最近一次被存取或被执行的时间，UNIX 时间戳，对应 POSIX stat.st_atime
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    lastModifiedTime
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                文件最后一次被修改的时间，UNIX 时间戳，对应 POSIX stat.st_mtime
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [isDirectory()](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                判断当前文件是否一个目录
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [isFile()](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                判断当前文件是否一个普通文件
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码


```js
const fileSystemManager = tt.getFileSystemManager();
fileSystemManager.stat({
    path: "ttfile://temp",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`stat fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "stat": {
        "mode": 16889,
        "size": 3488,
        "lastAccessedTime": 1637505688,
        "lastModifiedTime": 1637505688
    },
    "errMsg": "stat:ok",
}
``` 




