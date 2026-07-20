---
document_id: '7073817463670571014'
directory_id: '7072290825036890118'
title: 浮动图片指南
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/float-image-user-guide
breadcrumb:
- Server API
- Docs
- Sheets
- Floating image
- Float image user guide
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:41Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/float-image-user-guide
---

# 浮动图片使用指南

## 应用场景

在工作表内操作浮动图片

## 图片说明

表格只存储图片的 token，在创建浮动图片之前先上传图片至表格，参考[上传素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all)、[分片上传素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/multipart-upload-media/introduction)；查询和获取只返回图片的 token，下载图片请参考[下载素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download) 、[获取素材临时下载链接](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/batch_get_tmp_download_url)

## 支持的接口

同一张图片的 token 可以被放置在表格的不同位置，不同 token 的图片整表限制为4000，超过则不允许再创建浮动图片。支持通过以下接口管理浮动图片：

1. 【[获取浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get)】获取指定 id 的浮动图片的信息
2. 【[创建浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create)】根据传入的参数创建浮动图片
3. 【[更新浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch)】更新浮动图片的位置和宽高
4. 【[删除浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete)】删除浮动图片
5. 【[查询浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query)】查询子表内所有的浮动图片信息

## 浮动图片参数
### 示例
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5ba581c9134323943e6d5de6f45bc58e_Uw6DNDYQIG.png)

### **float_image_id**

子表内唯一标志浮动图片的 id。创建浮动图片时可选，不传会自动生成，如果传会校验有效性，需要满足：长度为10，由 0-9、a-z、A-Z 随机组成。

### **float_image_token**

浮动图片的 token，通过[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all)获得。在创建浮动图片时必填。

### **range**

浮动图片左上角所在单元格位置，只允许单个单元格的形式，如 "ahgsch!A1:A1"。在创建浮动图片时必填。


### **width**

浮动图片展示的宽度，创建浮动图片时不传会默认采用图片真实的宽度，如果传则需要大于等于20像素。

### **height**

浮动图片展示的高度，创建浮动图片时不传会默认采用图片真实的高度，如果传则需要大于等于20像素。

### **offset_x**

浮动图片左上角距离所在单元格左上角的横向偏移，默认为0，设置的值需要大于等于0、小于浮动图片左上角所在单元格的宽度。

### **offset_y**

浮动图片左上角距离所在单元格左上角的纵向偏移，默认为0，设置的值需要大于等于0、小于浮动图片左上角所在单元格的高度。

