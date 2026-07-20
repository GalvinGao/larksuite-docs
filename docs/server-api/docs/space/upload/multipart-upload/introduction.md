---
document_id: '7028022131297189893'
directory_id: '7026663934569070597'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/multipart-upload-file-/introduction
breadcrumb:
- Server API
- Docs
- Space
- Upload
- Multipart Upload
- Introduction
document_type: GuideDocumentType
updated_at: 2022-10-08T09:38:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/multipart-upload-file-/introduction
---

# 概述
该文档用于介绍分片上传文件API概览。

对于较大的文件以及网络中断性大的场景（大于20MB）推荐选择此上传方式。对文件进行定长分片上传，提高上传成功率，减少带宽使用，同时也能依据分片上传进度展示上传进度。该上传方式支持恢复上传，只需记录上传事务ID和对应上传块号，在申请上传事务ID的一天内是可以恢复上传的。

##  方法

###  [分片上传文件（预上传）](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_prepare)
发送初始化请求获取上传事务ID和分块策略，目前是以4MB大小进行定长分片。

###  [分片上传文件（上传分片）](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_part)
上传对应文件的分片。

###  [分片上传文件（完成上传）](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_finish)
触发完成上传。
