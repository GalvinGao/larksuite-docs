---
document_id: '7275897728244088838'
directory_id: '7273792780344934406'
title: 步骤二：启动本地服务
full_path: /home/quick-access-to-contact-api/step-2-start-the-local-service
breadcrumb:
- Home
- Quickly Integrate to Contacts
- 'Step 2: Start the local service'
document_type: GuideDocumentType
updated_at: 2023-09-07T02:00:14Z
source_url: https://open.larksuite.com/document/home/quick-access-to-contact-api/step-2-start-the-local-service
---

# 步骤二：启动本地服务

在本步骤，你将下载示例代码并运行。

## 操作步骤

1. 执行以下命令，下载[示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8510c808b3ff62efa4a620d0f835cfdc_DK78zc3e1v.zip)。

    ```PowerShell
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8510c808b3ff62efa4a620d0f835cfdc_DK78zc3e1v.zip -o oapi-contact.zip
    ```

2. 下载完成后，使用`unzip`命令进行解压。


    Windows 用户可以直接使用解压缩工具进行解压。


    ```PowerShell
    unzip oapi-contact.zip
    ```

  	解压后的项目目录结构如下：
    
    ```JavaScript
    oapi-contact
    ├── .gitignore
    ├── client.js
    ├── config.js
    ├── create-department.js
    ├── create-sub-department.js
    ├── find-all-department.js
    ├── get-department-info.js
    ├── index.js
    ├── listen-user-event.js
    ├── modify-department-name.js
    ├── package.json
    └── spinner.js
    ```

3. 执行以下命令，进入 oapi-contact 文件夹。

    ```PowerShell
    cd oapi-contact
    ```

4. 编辑 `config.js` 配置文件。修改文件中 **appId** 、 **appSecret** 和 **verificationToken** 的值为实际的应用凭证。

	- 方式一：在命令行通过 **vi/vim** 打开并编辑配置文件。命令示例：`vim config.js`。

	- 方式二：在本地设备中手动打开`/oapi-contact`文件夹，找到对应的`config.js`文件，使用常用的文本编辑器打开并编辑。
	
	应用凭证获取方式：
    
     * 应用的 **appId** 和 **appSecret** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **凭证与基础信息** 页查看。
      
      	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/be069cb96e07a3512076c0e6aba814db.png?height=1024&lazyload=true&maxWidth=600&width=2690)

     * 应用的 **Verification Token** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **事件订阅** 页查看。

        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d4c04f3997352d2c32b253fd275ec6a5_i5DsDmQTc2.png?height=522&lazyload=true&maxWidth=600&width=1470)


5. 继续在命令行中，执行以下命令，安装项目依赖。

    ```PowerShell
    npm install
    ```

6. 依赖安装成功后，执行以下命令，启动本地服务。

    ```PowerShell
    node index.js
    ```

  	成功的运行结果如下图所示：
  
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/111146c7ebb9a89f6a7c4a9c95e5ef01_Apc7QQzP5D.png?height=174&lazyload=true&maxWidth=600&width=1504)

