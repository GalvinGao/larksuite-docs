---
document_id: '7275897728244285446'
directory_id: '7273792780344918022'
title: 步骤三：启动本地服务
full_path: /home/automatic-attendance-management-based-on-approval/step-3-start-the-local-service
breadcrumb:
- Home
- Auto Attendance Management
- 'Step 3: Start the local service'
document_type: GuideDocumentType
updated_at: 2023-09-07T07:07:46Z
source_url: https://open.larksuite.com/document/home/automatic-attendance-management-based-on-approval/step-3-start-the-local-service
---

# 步骤三：启动本地服务

在本步骤，你将下载并启动教程提供的示例代码。

## 操作步骤
1. 使用本地的命令行工具，下载[示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c6058ea04adc387394794715c8174527_1nsOggWvDU.zip)并解压。

    ```PowerShell
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c6058ea04adc387394794715c8174527_1nsOggWvDU.zip -o approval-demo.zip
    ```

2. 下载完成后，使用`unzip`命令进行解压。


    Windows 用户可以直接使用解压缩工具进行解压。


    ```PowerShell
    unzip approval-demo.zip
    ```

    解压后的项目目录结构如下：

    ```JavaScript
    ├── src
    │   ├── config.js  -----  配置项，开发者需要在此文件上修改内容
    │   ├── approval.js  ---  审批相关代码
    │   ├── attendce.js  ---  考勤打卡相关
    │   └── index.js  ------  项目入口
    ├── package.json  ------  npm package
    └── yarn.lock     ------  yarn lock 文件
    ```

3. 执行以下命令，进入 `approval-demo` 文件夹。

    ```PowerShell
    cd approval-demo
    ```

4. 执行以下命令修改 `config.js` 配置文件。


    Windows用户可直接使用记事本进行修改。


    ```PowerShell
    vi src/config.js
    ```

   1. 按 `i` 进入编辑模式，修改文件中的配置信息。 
      * 应用的 **APP_ID** 和 **APP_SECRET** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **凭证与基础信息** 页查看。

        ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/34daba0c7c04a7e6c460effa8da58ee3.png?height=1026&lazyload=true&maxWidth=600&width=2590)

      * **CUSTOM_APPROVE_CODE** 可以在审批实例编辑页查看。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e55feccbd6b1d6f7dbbbc5645fe33381_WYcJpKVYxn.png?height=742&lazyload=true&maxWidth=600&width=2334)


   2. 修改完成后，按 `:wq` 保存退出。

<br>

5. 执行以下命令，安装项目依赖。

    ```PowerShell
    npm install
    ```

6. 依赖安装成功后，执行以下命令，启动本地服务。

    ```PowerShell
    npm run dev
    ```

	成功的运行结果如下图所示：
  
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d046e75c3ad521a361d3c3e9f135b195_GxXRIntTww.png?height=212&lazyload=true&maxWidth=600&width=854)

