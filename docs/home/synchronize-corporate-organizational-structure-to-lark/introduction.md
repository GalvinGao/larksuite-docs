---
document_id: '7074952334765817862'
directory_id: '7073442394955546629'
title: 简介
full_path: /home/synchronize-corporate-organizational-structure-to-feishu/synchronize-corporate-organizational-structure-to-feishu
breadcrumb:
- Home
- Synchronize corporate organizational structure to Lark
- Introduction
document_type: GuideDocumentType
updated_at: 2023-05-16T08:43:48Z
source_url: https://open.larksuite.com/document/home/synchronize-corporate-organizational-structure-to-feishu/synchronize-corporate-organizational-structure-to-feishu
---

# 简介

如果你的企业已经在使用 HR 系统维护核心人力资源，可以通过编写简单的同步程序，利用Lark提供的通讯录 OpenAPI 将需要使用Lark的部门/用户自动同步到Lark。

这样，企业 HR 无需关注Lark组织架构，继续使用熟悉的内部 HR 系统维护人员数据即可。

##  示例效果
以下图中的企业组织架构为例：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c75c36544c9a7d993128486d645d1bf1_xldZVjhGOU.png?lazyload=true&width=1640&height=707)
本教程将引导实现企业原有的部门和人员同步至Lark的组织架构中。


## 创建一个应用
你需要先创建一个“企业自建应用”，才能够获得调用更新通讯录API的相关权限。创建方法如下。

1. 进入[Lark开放平台—开发者后台](http://open.larksuite.com/app)点击“创建自建应用”。  

    ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/196ce1122b5522ed626540ac7812f57f_M1R0EgRqpb.png?lazyload=true&width=2482&height=1438)

:::html
<md-td>
2. 填写应用信息，应用名称为 `Contact Test` 

    <img src=//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b25bb9ea3a83ab3f3dd59c1e12167192_ywSICagOFe.png width=60%/>
</md-td>
:::
3. 创建成功后，可以看到 `Contact Test` 应用被添加到企业自建应用目录中，点击进入应用详情页可以看到新创建的应用。

    ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9e10c79935a62a438bff0d33434361fb_TpehTKLQqv.png?lazyload=true&width=2284&height=1512)
