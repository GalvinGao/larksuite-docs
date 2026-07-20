---
document_id: '6965400907878301701'
directory_id: '6920532209304993793'
title: 获取通讯录权限范围说明
full_path: /ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority
breadcrumb:
- Server API
- Contacts
- Contact Scope Description
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:18Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority
---

# 获取通讯录权限范围说明
## 1.什么是“获取通讯录权限范围”？
应用 **获取通讯录权限范围** 定义了应用调用通讯录接口能获取哪些部门和用户数据，不在应用获取通讯录范围内的数据是无法访问的。默认情况下，通讯录权限范围与应用的可用范围一致。<br>举例来说：假设通讯录中有 A、B、C 三个部门，若一个应用只有 B 和 C 两个部门的通讯录权限，那么该应用就只能通过接口获取到 B 和 C 两个部门的数据，获取 A 部门就会报出没有权限的错误。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5855593982277edc5f894b0f5400e191_SQvowGMBUY.png?lazyload=true&width=1280&height=704)
**举个实践案例：**<br>
小王开发了一个人力管理相关的自建应用。出于数据保密的要求，这个应用只能被HR同事使用。但由于应用需要管理全公司的人事信息，需要获取全公司的通讯录数据。<br>
小王可以将这个应用的可用范围设置为HR部门的同事，同时申请应用的通讯录权限范围为全部成员。应用管理员审核通过后，这个人力管理应用就可以控制比较小的可用人员范围，同时又能获得全公司的通讯录数据了。



## 2.如何设置“获取通讯录权限范围”？

### 对于租户管理员
由 租户超级管理员 或 安装该应用的租户管理员 在 [管理后台/工作台/应用管理](http://www.larksuite.com/admin/appCenter/manage) 进入某应用配置详情页进行设置。

<br>具体设置方法参见下图：


![3e94a48448c949dd03f0317cefa4a447_Wbbu625lBG (1).gif](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/13291ea9d6f2c797af53f511df08f469_NB1fPqjLrc.gif?lazyload=true&width=795&height=436)

### 对于应用开发者
可以在 [开发者后台](https://open.larksuite.com/app) > 打开你的应用 > 权限管理面板中设置通讯录权限范围。具体设置方法参见下图：

![contact.gif](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/018c68e7ab70c5ddb733566ae1a19ad1_YoIeoDkRn6.gif?lazyload=true&width=3486&height=1940?lazyload=true&width=2000&height=1113)

:::warning
注意：
- 默认情况下，通讯录权限范围与应用的可用人员范围一致
- 修改通讯录权限范围后，需要创建应用版本，由管理员审核通过后才可生效
- 通讯录数据访问范围需要配合通讯录权限才可生效，请注意在权限配置中申请“通讯录”板块下必要的权限

:::

## 一些概念区分
**注意：应用获取通讯录权限范围容易和下列概念混淆，请注意区分。**
- **应用可用范围**<br>应用可用范围定义了哪些人可以使用该应用，由 安装该应用的租户管理员 / 超级管理员 在管理后台设置，可以按部门、用户组、用户设置。<br>如果企业希望应用服务于哪些人就可以获取哪些人的数据，则通过在后台打开“与可用人员范围保持一致”实现应用“获取通讯录权限范围”同步自“应用可用范围”，届时，应用的“获取通讯录权限范围”将和“应用可用范围完全一致”，不可单独修改。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3d7efabfe2917a1b03f930d1b5fe426f_ZD7A5yo2PY.png?lazyload=true&width=1640&height=855)

- **组织架构可见范围：**<br>组织架构可见范围定义了用户在Lark / 通讯录 / 组织架构中可见的组织架构范围，由租户管理员在 [管理后台 / 安全 / 用户权限 / 组织架构可见范围](http://www.larksuite.com/admin/security/permission/visibility) 中设置，可以理解为针对用户的通讯录可访问范围管控，而应用“获取通讯录权限范围”是针对应用的通讯录可访问范围，二者不是一个维度的管控。


## 3.获取通讯录权限范围的分类

### 用户通讯录范围权限

需要用户通讯录范围权限的接口有：用户的delete，update，patch，get方法，用户组成员的添加和删除方法。<br>
只要有一个部门的通讯录范围权限，那么就拥有这个部门下所有用户的权限。

### 部门通讯录范围权限

新版的通讯录中，将部门的通讯录权限细化为父部门权限和部门权限两种。<br>
需要父部门通讯录范围权限的接口有：部门的create，delete，list，update，patch，parent方法。<br>
需要部门通讯录范围权限的接口有：部门delete，update，patch，get方法，用户的create、delete、update、patch、list方法。<br>
只要有一个部门的通讯录范围权限，那么就拥有这个部门下所有子部门的权限。

### 全员权限
全员权限就是拥有通讯录下所有数据的权限。以下几种情况需要拥有全员权限:
- 部门的create方法在根部门上创建子部门
- 部门的update和patch方法将父部门更新为根部门
- 部门的list方法获取根部门下的子部门
- 部门的delete方法删除根部门下的子部门
- 部门的get方法获取根部门信息
- 用户的create方法在根部门下创建用户
- 用户的delete方法删除根部门下的用户
- 用户的update和patch方法将所属部门更新为根部门
- 用户的list方法获取根部门下的用户
- 用户组的create方法

