---
document_id: '7073692582769213446'
directory_id: '7073451436033851397'
title: Stats
full_path: /uYjL24iN/uETOuETOuETO/stat/stats
breadcrumb:
- Client API
- Web app/Gadget API
- File
- Stats
- Stats
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/stat/stats
---

# Stats


描述文件状态的对象。

## 属性

| 名称             | 数据类型 | 描述                                                                        |
| ---------------- | -------- | --------------------------------------------------------------------------- |
| mode             | string   | 文件的类型和存取的权限，对应 `POSIX` `stat.st_mode`                         |
| size             | number   | 文件大小，单位：`B`，对应 `POSIX` `stat.st_size`                            |
| lastAccessedTime | number   | 文件最近一次被存取或被执行的时间，UNIX 时间戳，对应 `POSIX` `stat.st_atime` |
| lastModifiedTime | number   | 文件最后一次被修改的时间，UNIX 时间戳，对应 `POSIX` `stat.st_mtime`         |

## 方法

| 名称                                                                    | 参数 | 返回值  | 描述                         |
| ----------------------------------------------------------------------- | ---- | ------- | ---------------------------- |
| [isDirectory](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory) | 无   | boolean | 判断当前文件是否一个目录     |
| [isFile](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file)           | 无   | boolean | 判断当前文件是否一个普通文件 |



## 代码示例


```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    const tempFile = res.tempFilePaths[0];
    fileSystemManager.stat({
      path: tempFile,
      success(res) {
        const stat = res.stat;
        console.log("mode:", stat.mode);
        console.log("size:", stat.size);
        console.log("lastAccessedTime:", stat.lastAccessedTime);
        console.log("lastModifiedTime:", stat.lastModifiedTime);
        
        console.log("isDirectory:", stat.isDirectory());
        console.log("isFile:", stat.isFile());
      },
      fail(res) {
        console.log("调用失败", res.errMsg);
      },
    });
  },
});

```
