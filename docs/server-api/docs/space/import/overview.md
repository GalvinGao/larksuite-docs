---
document_id: '7031483915610456069'
directory_id: '7031445675032199173'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide
breadcrumb:
- Server API
- Docs
- Space
- import
- Overview
document_type: GuideDocumentType
updated_at: 2023-05-24T02:40:04Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide
---

# 导入使用指南
本文介绍如何将指定格式的文件导入为新版文档、电子表格以及多维表格等，并挂载到云空间特定目录下。<br>
## 步骤一：上传本地文件
:::note
如果需要导入的文件已上传至云空间，则可跳过本步骤。
:::
目前支持以下上传方式：<br>
**方式一：** 
通过[上传素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all)接口上传导入素材，请求涉及的主要参数有：
- `parent_type`：需传入固定值`ccm_import_open`；<br>
- `parent_node`：无需填写；<br>
- `extra`：参考[素材概述-上传点类型及对应 Extra 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction#5e8f27c1)进行填写。
<br>

以将名为 "demo.txt" 的本地文件导入为新版文档为例：
```JavaScript 
curl --location --request POST 'https://{domain}/open-apis/drive/v1/medias/upload_all' \ // domain 需替换为真实域名
--header 'Content-Type: multipart/form-data' \
--header 'Authorization: Bearer u-{xxxxx}' \ // 需替换为真实 Authorization
--form 'file_name="demo.txt"' \ // 本地文件名
--form 'parent_type="ccm_import_open"' \ // 使用固定值：ccm_import_open
--form 'size="5"' \ // 本地文件大小（以字节为单位）
--form 'extra={"obj_type":"docx","file_extension":"txt"}' \ // 将本地 txt 格式文件导入为新版文档
--form 'file=@"demo.txt"' // 本地文件路径
``` 


**方式二：** 通过[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all)接口上传文件。
:::note
对于上述两种方式，开发者可以按需选择：
- 方式一：上传的素材是临时文件，在完成导入后，系统会主动删除；<br>
- 方式二：上传的文件是上传到云空间，文件归属上传者，占用上传者云空间用量，导入系统不会做主动删除。
:::

## 步骤二：创建导入任务
调用[创建导入任务](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create)接口，请求成功将返回`ticket`（导入任务 ID），请求涉及的主要参数有：
- `file_token`：要导入的素材或文件的`Token`；<br>
- `point`：云空间挂载点，其中`mount_key`是目标挂载目录，如为空，则挂载到导入者的云空间根目录下；<br>
- `type`：导入后生成的云文档类型，目前支持生成以下类型：
  + `docx`：新版文档
  + `sheet`：电子表格
  + `bitable`：多维表格
  + `doc`：旧版文档 ==已不推荐使用，请使用新版文档==
- `file_extension`: 需要导入的文件后缀类型，即`file_token`对应文件的类型。
:::note
`type`和`file_extension`需要结合来看，其中：
- 当`type`取值为`docx`时，`file_extension`支持取值为：
  + `docx`（Microsoft Word Document）
  + `doc`（Microsoft Word 97-2004 Document）
  + `txt`（文本文件）
  + `md`（Markdown）
  + `mark`（Markdown）
  + `markdown`（Markdown）
  + `html` (HTML)
- 当`type`取值为`sheet`时，`file_extension`支持取值为：  
  + `xlsx`（Microsoft Excel Workbook）
  + `xls`（Microsoft Excel 97- Excel 2003 Workbook）
  + `csv`（Comma Separated Values）
- 当`type`取值为`bitable`时，`file_extension`支持取值为：  
  + `xlsx`（Microsoft Excel Workbook）
  + `csv`（Comma Separated Values）
- 当`type`取值为`doc`时，`file_extension`支持取值为：  
  + `docx`（Microsoft Word Document）
  + `doc`（Microsoft Word 97-2004 Document）
  + `txt`（文本文件）
  + `md`（Markdown）
  + `mark`（Markdown）
  + `markdown`（Markdown）
:::
以将一个`file_extension`为 ".txt" 且`Token`为 "Nlg1b8TZQoVDy8xcSzJcPPjinzf" 的纯文本文件，导入为新版文档为例：
```javascript 
curl -i -X POST 'https://{domain}/open-apis/drive/v1/import_tasks' \ // domain 需替换为真实域名
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer u-{xxxxx}' \ // 需替换为真实 Authorization
-d '{
	"file_extension": "txt", // 源文件为 "txt" 格式
	"file_name": "demo",
	"file_token": "Nlg1b8TZQoVDy8xcSzJcPPjinzf",
	"point": {
		"mount_key": "VsGvfNmCAl3t62dIppwcGohNnzc",
		"mount_type": 1 // 目前只支持挂载到云空间
	},
	"type": "docx" // 导入为新版文档
}'
``` 
## 步骤三：查询导入任务结果
调用[查询导入任务结果](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/get)接口，根据步骤二返回的`ticket`（导入任务 ID）按需轮询导入结果，响应涉及的主要参数有：
- `job_status`：导入任务状态，比如`0`表示导入任务执行成功；
- `extra`：额外信息，有时导入虽然成功，但由于源文件内容超过了系统上限，进而被系统截断，此时会返回该参数，进行额外的提示：
  + **2000**：表格列数超过最大限制，超出的部分将被截断丢弃；
  + **2001**：表格元格数超过最大限制，超出的部分将被截断丢弃；
  + **2002**：表格图片超过 4000 张，超出的图片被丢弃；
  + **2003**：云空间存储空间不足，请联系企业管理员；
  + **2004**：表格部分图片上传失败；
  + **2005**：表格单元格字符长度超过最大限制，超出的部分将被截断丢弃；
  + **3000**：多维表格图片超出数据行区域会丢弃；
  + **3001**：多维表格图片超出数据列区域会丢弃；
  + **3003**：多维表格列数超过最大限制，超出的部分将被截断丢弃；
  + **3004**：多维表格单元格数超过最大限制，超出的部分将被截断丢弃；
  + **3005**：多维表格图片超过 4000 张，超出的图片被丢弃；
  + **3006**：多维表格部分图片上传失败。
