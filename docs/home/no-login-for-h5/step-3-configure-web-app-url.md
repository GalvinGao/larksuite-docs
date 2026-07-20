---
document_id: '7233232281316786182'
directory_id: '7073442394955579397'
title: 步骤三：配置网页应用访问地址
full_path: /home/quickly-create-a-login-free-web-app/configure-the-web-application-access-address
breadcrumb:
- Home
- No-login for H5
- 'Step 3: Configure web app URL'
document_type: GuideDocumentType
updated_at: 2023-09-01T06:36:47Z
source_url: https://open.larksuite.com/document/home/quickly-create-a-login-free-web-app/configure-the-web-application-access-address
---

# 步骤三：配置网页应用访问地址

在本步骤，你将在开发者后台配置网页应用的桌面端主页和移动端主页。
:::html
<md-td>
## 操作步骤
1. 打开[开发者后台](https://open.larksuite.com/app)，进入应用详情页。
2. 在 **网页应用** 页面，修改 **网页配置**，**桌面端主页** 和 **移动端主页** 都填写为步骤二中获取的临时内网访问地址。例如，在本文中为`http://10.4.143.108:3000` 。

	正式上线时，此处配置主页地址需为 **公网地址** 。为了快速体验接入流程，本示例中暂时先使用本地环境。

    <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/715153e053101934bd9f5f7e80565987.png?lazyload=true&width=2230&height=1268" style="width:60%"/>
  
3. 单击进入 **安全设置** 页面，添加 **重定向 URL** 和 **H5可信域名**，地址为服务端访问地址加斜杠，本文设置为 `http://10.4.143.108:3000/`。
    

    **说明：**
    * 为了Lark客户端内网页应用的安全可信，仅可信域名内的 H5 可以调用 JSAPI 获取数据，因此需要为应用配置可信域名。
    * 重定向URL的末尾需要有斜杠（/）。

    
    <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/5e5c9bf6ae0fd7cf4db2ee25de563781.png?lazyload=true&width=2800&height=1624" style="width:60%"/>

</md-td>
:::
