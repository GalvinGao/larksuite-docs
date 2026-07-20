---
document_id: '7074952334765834246'
directory_id: '7073442394955546629'
title: 创建和更新部门
full_path: /home/synchronize-corporate-organizational-structure-to-feishu/create-and-update-departments
breadcrumb:
- Home
- Synchronize corporate organizational structure to Lark
- Create and update departments
document_type: GuideDocumentType
updated_at: 2023-05-16T08:44:02Z
source_url: https://open.larksuite.com/document/home/synchronize-corporate-organizational-structure-to-feishu/create-and-update-departments
---

# 创建和更新部门

本文将以API调试台工具为例，调用通讯录相关接口创建和更新部门。

## 创建部门

1. 打开[API调试台](https://open.larksuite.com/api-explorer)工具。

2. 在左侧API列表依次选择**通讯录** > **部门** > **创建部门**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/23ef9e6ff1c8d69eba51362a24d913c8_F8pbKydWr6.png?lazyload=true&width=2494&height=1474)
3. 在请求体中填入部门信息，其中`leader_user_id`为选填，可先确立部门架构后再完善部门信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2fe8c4879320211a15b4d5a09681d0a2_rPbNrRhfqB.png?lazyload=true&width=2756&height=1530)
4. 单击右上角**开始调试**。
    
    调用成功后**调试结果**中会返回所创建的部门信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/946a3fed86eec92deda96dee40cf9b5e_PW7rmXBC0k.png?lazyload=true&width=2770&height=1524)
5. 重复上述操作，直至所有部门都创建成功。
    
    :::note
    创建子部门时，`parent_department_id`需填写已经创建的父部门的`open_department_id`。
    :::
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/68802b6852f225ead43ea6bad9c77644_zWWc3Nnrfo.png?lazyload=true&width=2754&height=1518)

## 更新部门信息

1. 在API调试台左侧的API列表中，依次选择**通讯录** > **部门** > **更新部门所有信息**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8dcde761b7c85050a3da481c39634e04_hQ0YJmmkn4.png?lazyload=true&width=2098&height=1548)
2. 在路径参数中填写需要修改的部门ID信息。
    
    根据你填写的部门ID类型的不同，你还需要修改查询参数中的**department_id_type**为对应的ID类型。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b2cf8d3467f751217961236512b0fd58_625bn6WZsc.png?lazyload=true&width=1786&height=652)
3. 在请求体中，填入需要修改的部门信息，最后单击右上角**开始调试**按钮。
    
    调用成功后，**调试结果**中会返回，发生变更的部门信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/30081b3d317d73bf1adf92729aea8703_Qt9iT32Js7.png?lazyload=true&width=2756&height=1544)
