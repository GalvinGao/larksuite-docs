---
document_id: '7236573236108476422'
directory_id: '7234348439831265285'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/export-user-guide
breadcrumb:
- Server API
- Docs
- Space
- export
- Overview
document_type: GuideDocumentType
updated_at: 2023-05-24T02:40:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/export-user-guide
---

# 导出使用指南

本文介绍如何将新版文档、电子表格、多维表格和旧版文档导出为本地文件，以及导出的相关注意事项。

## 导出步骤
### 步骤一：创建导出任务
调用[创建导出任务](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create)接口，将云文档导出为指定格式的本地文件，目前支持对新版文档、电子表格、多维表格和旧版文档进行导出。该接口为异步接口，任务创建完成即刻返回，并不会阻塞等待到任务执行成功，请求涉及的主要参数有：
- `type`：要导出的云文档类型，目前支持以下类型云文档：
  + `docx`：新版文档
  + `sheet`：电子表格
  + `bitable`：多维表格
  + `doc`：旧版文档
- `file_extension`：导出产物的文件扩展名，比如`pdf`。
:::note
`type`和`file_extension`需要结合来看，即导出产物的文件扩展名须和云文档类型相匹配，其中：
- 当`type`取值为`docx`时，`file_extension`支持取值为：
  + `docx`（Microsoft Word Document）
  + `pdf`（Portable Document Format）
- 当`type`取值为`sheet`时，`file_extension`支持取值为：
  + `xlsx`（Microsoft Excel Workbook）
  + `csv`（Comma Separated Values）
- 当`type`取值为`bitable`时，`file_extension`支持取值为：
  + `xlsx`（Microsoft Excel Workbook）
  + `csv`（Comma Separated Values）
- 当`type`取值为`doc`时，`file_extension`支持取值为：
  + `docx`（Microsoft Word Document）
  + `pdf`（Portable Document Format）
:::
:::warning
如果要导出`wiki`（知识库）文档，需要先通过[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)得到节点对应的`obj_token`（文档 Token）和`obj_type`（文档类型），然后据此来创建具体的导出任务。
:::
### 步骤二：查询导出任务结果
调用[查询导出任务结果](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get)接口，通过要导出文档的`token`和`ticket`（导出任务 ID）获取导出结果。
### 步骤三：下载导出文件
调用[下载导出文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/download)接口，根据导出任务结果中获取到的导出产物 `token` 下载对应的结果文件。


## 注意事项
- 导出的文件会在任务结束 10 分钟后删除，删除后将无法下载；
- 导出为`docx`时，会受文档资源（如图片）大小限制，如果文档内资源总计超过 1GB，将会导出失败；
- 导出为`pdf`时，会受文档资源（如图片）大小限制，如果文档内资源总计超过 128MB，将会导出失败；
- 当企业内部启用水印时：
  + 使用应用身份进行导出，将会使用应用 ID 作为水印，比如`cli_a2c2xxxxxxxxd01b`；
  + 使用用户身份进行导出，将会使用用户标识作为水印。
