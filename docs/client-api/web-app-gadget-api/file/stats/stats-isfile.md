---
document_id: '7073692582768967686'
directory_id: '7073451436033851397'
title: Stats.isFile
full_path: /uYjL24iN/uETOuETOuETO/stat/stats_is_file
breadcrumb:
- Client API
- Web app/Gadget API
- File
- Stats
- Stats.isFile
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file
---

# Stats.isFile()


判断当前文件是否一个普通文件。



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.11.0+</md-version> | <md-version>V4.11.0+</md-version> | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.11.0+</md-version> | <md-version>V4.11.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入
无



## 输出


返回值：boolean


## 示例代码


```js
const fileSystemManager = tt.getFileSystemManager();
fileSystemManager.stat({
    path: "ttfile://temp",
    success(res) {
      console.log(res.stat.isFile());
    },
    fail(res) {
      console.log(`stat fail: ${JSON.stringify(res)}`);
    }
});
```
返回值示例：

```json
false
``` 

