---
document_id: '7233612551991672838'
directory_id: '7199928167142244357'
title: 准备工作
full_path: /home/sales-statistics-base-on-spreadsheets/prep-work
breadcrumb:
- Home
- Sales statistics base on spreadsheets
- Prep work
document_type: GuideDocumentType
updated_at: 2023-05-16T03:12:31Z
source_url: https://open.larksuite.com/document/home/sales-statistics-base-on-spreadsheets/prep-work
---

# 准备工作

1、在[开发者后台](https://open.larksuite.com/app/)**新建企业自建应用**，点击应用名称进入应用详情页。

2、点击**凭证与基础信息**切换页面，拿到 `App ID` 和 `App Secret` 值。

3、点击**权限管理**切换页面，搜索需要的**权限配置**，并前往开发者后台[申请权限](https://open.larksuite.com/app)。
<br>
- **依赖权限清单**
  - **查看、评论、编辑和管理云空间中所有文件** `drive:drive`
  - **查看、评论、编辑和管理电子表格** `sheets:spreadsheet`

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/07b5a4b1b4df052ebdbd15b153237ea0_lVguHoxlti.png?lazyload=true&width=1280&height=339)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/04be8834f0d1fc0d86d245fccee48145_IsNAp0tI3P.png?lazyload=true&width=1280&height=334)

4、拉取最新代码到本地，并进入 **python** 目录。

**Mac/Linux**
```
curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/89f30e7d3a55a9058ef7b88713f69687_yGllDrcPYj.zip -o sheets_quick_start.zip
unzip sheets_quick_start.zip
cd sheets_quick_start/python
```
**Windows**
```
curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/89f30e7d3a55a9058ef7b88713f69687_yGllDrcPYj.zip -o sheets_quick_start.zip
sheets_quick_start.zip
cd sheets_quick_start/python
```
5、修改环境值，修改 `.env` 文件中应用凭证数据为真实数据，和其他应用运行相关参数。

```
APP_ID=cli_a2cb64xxxx38900c
APP_SECRET=wjwB4rxxxxEhxg6q0TUuCbJiYJUtQ3sh
EMAIL=xxxx@email.com
```
