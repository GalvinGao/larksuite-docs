---
document_id: '7275897728244400134'
directory_id: '7273792780344885254'
title: 步骤三：体验效果
full_path: /home/quick-access-to-base/step-3-experience-the-effect
breadcrumb:
- Home
- Quickly Integrate to Base
- 'Step 3: Experience the effect'
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:54Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/step-3-experience-the-effect
---

# 步骤三：体验效果

:::note
对于测试版应用来说，仅测试企业的 **创建者** 及 **测试人员** 可以访问该应用。
:::


1. 创建多维表格。

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3655d064b65f5dcb9c4ddafc0e830b68_EpHqprvqgH.png?height=736&lazyload=true&maxWidth=600&width=2312)

    - 保存 `app_token` 参数值，后续的多维表格管理操作均需要使用该参数值进行配置。
  
    - 通过命令行回显的 url 访问该多维表格，查看到的多维表格示例如下图所示。
  
      ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0d9203138df6930b086decbbcbf086de_45zZ6XRiXT.png?height=768&lazyload=true&maxWidth=600&width=2394)

2. 添加一张表。

   	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1a8357b516b92953edc17ce96969c117_cGJeSBu6Jt.png?height=824&lazyload=true&maxWidth=600&width=1742)

    - 在添加表时，您需要填写 appToken、表名。
  
    - 添加完成后，您需要保存 table_id 信息，用于后续添加记录、获取记录。
    
  	进入多维表格 url，查看添加的表信息。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/10d36c371812ce7bdfe7ad4fa84c959e_TKRCZ4Jycq.png?height=286&lazyload=true&maxWidth=600&width=1968)

3. 添加表记录。
 
   添加表记录时，您需要填写 appToken、table_id。
   

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5705fc97dbf55d43df80020e04b81450_y6JoU1SmVF.png?height=610&lazyload=true&maxWidth=600&width=2172)

    进入多维表格 url，查看添加的表记录。
  
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f37b557d93df8520ae21d17f702a2494_P93RZ1iTYp.png?height=962&lazyload=true&maxWidth=600&width=1958)

4. 删除表记录。
  
   删除表记录时，需要获取待删除记录的 `record_id`（该参数值可以在添加表记录后的回显信息中获取），然后依次填写 appToken、table_id、record_id。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fae2840f1d985ef1d88d7950548fb309_in5WQEtr2X.png?height=334&lazyload=true&maxWidth=600&width=1934)

    您可以进入多维表格 url，查看指定 record_id 对应的表记录已被删除。
  
5. 填写 appToken、table_id，将指定的表格导出本地。
  
     该部分示例代码仅模拟表格导出操作。

     ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b89566fde2f1fc879a0412efd0831b2d_YC4uOMsNjy.png?height=1460&lazyload=true&maxWidth=600&width=1510)
