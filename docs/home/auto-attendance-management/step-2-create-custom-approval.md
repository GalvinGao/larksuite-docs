---
document_id: '7275897728244170758'
directory_id: '7273792780344918022'
title: 步骤二：创建自定义审批实例
full_path: /home/automatic-attendance-management-based-on-approval/step-2-create-a-custom-approval
breadcrumb:
- Home
- Auto Attendance Management
- 'Step 2: Create custom approval'
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:46Z
source_url: https://open.larksuite.com/document/home/automatic-attendance-management-based-on-approval/step-2-create-a-custom-approval
---

# 步骤二：创建自定义审批实例

在本步骤，你将创建一个名为 **带薪休假审批** 的自定义审批实例，用于发起带薪休假审批。

## 操作步骤

1. 登录[审批管理后台](https://www.larksuite.com/approval/admin)。
2. 浏览器地址栏链接上，添加参数`?devMode=on`开启开发者模式。

  	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/079e79625ee0d85c1f6e206c41ba8e58_JgpsbZwkZP.png?height=842&lazyload=true&maxWidth=600&width=1800)

3. 选择 **审批管理** ，单击 **创建审批** ，选择 **创建自定义审批** 。

  	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5f09fa1757a68ebe7de207093e73a29a_VOt0gK3bMl.png?height=1310&lazyload=true&maxWidth=600&width=2816)

4. 在**基础信息**页，填写审批流程 **名称**。
  
  	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/255fae5bac5fece8cd76e3cc32d21eae.png?height=1670&lazyload=true&maxWidth=600&width=2612)

5. 在 **表单设计** 页，添加 **日期区间** 组件，并将 **自定义 ID** 设置为 `timeInfo`，**日期格式** 设置为 **年-月-日 上午/下午（天）**。
  
	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7cc99d03f3ca81f8db83c87177247b43.gif?height=1176&lazyload=true&maxWidth=600&width=1762)


6. 在 **表单设计** 页，添加 **多行文本** 组件，并将 **自定义 ID** 设置为 `reason`。

  	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b8bb4a56354199fa54068b849f3acf13_aI0jaM2Hji.gif?height=1178&lazyload=true&maxWidth=600&width=1760)

7. 在 **流程设计** 页，单击 **审批人** 节点，将流程设置为自动通过。
  
  	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/0c1ab87373c6c5635a4eef0ee1a92877.png?height=681&lazyload=true&maxWidth=600&width=1280)

8. 流程设计完成后，单击 **发布** 。
  
  	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/dbbe85efb7f59592b624008cd90a6f48.png?height=1674&lazyload=true&maxWidth=600&width=2600)


