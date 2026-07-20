---
document_id: '7233612551991574534'
directory_id: '7199928167142227973'
title: 步骤四：下载并运行示例代码
full_path: /home/management-weekly-report-based-docs/run
breadcrumb:
- Home
- Manage Weekly Report with Docs
- 'Step 4: Download and run the sample code'
document_type: GuideDocumentType
updated_at: 2023-11-08T06:35:35Z
source_url: https://open.larksuite.com/document/home/management-weekly-report-based-docs/run
---

# 步骤四：下载并运行示例代码

在本步骤，你将下载并运行教程提供的示例代码。示例代码使用 Python 语言编写，请确保你已安装 Python 运行环境。

## 步骤一：下载示例代码

  
1. 执行以下命令，下载示例代码到本地。
   
  	* Mac OS 或 Linux执行以下命令。
      
        ```PowerShell
        curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c9d1ecc1ee8c67765a6d62d3d630321.zip -o weekly_quick_start.zip
        unzip weekly_quick_start.zip
        cd weekly_quick_start/python
        ```

	* Windows执行以下命令。
      
        ```PowerShell
        curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c9d1ecc1ee8c67765a6d62d3d630321.zip -o weekly_quick_start.zip
        weekly_quick_start.zip
        cd weekly_quick_start/python
        ```

2. 修改`.env`文件中的应用凭证为测试应用的凭证数据。
    
    - 方式一：在命令行通过 **vi/vim** 打开并编辑配置文件。命令示例：`vim .env`。

	- 方式二：在本地设备中手动打开`/weekly_quick_start/python`文件夹，找到对应的`.env`文件，使用常用的文本编辑器打开并编辑。
    
	应用凭证获取方式：
    
    - **APP_ID** 和 **APP_SECRET** 应用凭证信息可以在[开发者后台](https://open.larksuite.com/app)的 **凭证与基础信息** 页查看。

		![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/be069cb96e07a3512076c0e6aba814db.png?height=1024&lazyload=true&maxWidth=600&width=2690)

    	
    - **TEMPLATE_DOC_URL** 为模版文档链接。

		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d7ff86be68c02349cbe0067f97c9d3ae_bUiMKlwX54.png?height=692&lazyload=true&maxWidth=600&width=2378)


    - **WIKI_SPACE_ID** 可以在知识库 **设置** 页面获取。
			
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/757dfcedaa169b6e0629f5bd885b2140_gEqfQqrJ5B.png?height=1718&lazyload=true&maxWidth=600&width=2350)


    - **WIKI_NODETOKEN** 可以在文档页面获取。
			

        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bed55886e86b59dab4a23a96d7ae679e_i8IG6puGr2.png?height=664&lazyload=true&maxWidth=600&width=2342)  
    

    私有化部署时要修改`.env`文件中 **Lark_HOST** 为私有化部署所用的域名。
    
  
3. 打开`weekly.cron`修改cron表达式的参数。
  
	以下cron表达式表示：每个星期五下午十六点进入 `/home/app` 目录，运行 *weekly*.py 文件。

    ```PowerShell
    # Create a weekly report every Friday at 16:00
    0 16 * * Fri cd /home/app && /usr/local/bin/python weekly.py >> weekly.log 2>&1
    ```


## 步骤二：运行示例代码
你可以参考以下两种方式运行示例代码。

### Docker运行
如果您已有[Docker](https://www.docker.com/)环境，那么直接执行以下命令即可。
* Mac OS或Linux执行以下命令：

    ```PowerShell
    sh exec.sh
    ```

* Windows执行以下命令：

    ```PowerShell
    .\exec.ps1
    ```


### 本地运行
1. 执行以下命令，创建并激活Python虚拟环境。
   * Mac OS 或 Linux执行以下命令。

      ```PowerShell
      python3 -m venv venv
      . venv/bin/activate
      ```

   * Windows执行以下命令。

      ```PowerShell
      python3 -m venv venv
      venv\Scripts\activate
      ```

2. 执行以下命令，安装代码依赖。

    ```PowerShell
    pip3 install -r requirements.txt
    ```

3. 依赖安装成功后，执行以下命令运行示例代码。

    ```PowerShell
    python3 server.py
    python3 weekly.py
    ```
4. 打开浏览器，访问 `http://127.0.0.1:3000` 登录拥有知识库管理员账号，添加应用为知识库管理员。

	![](http://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/72e27ba03eddce93cc6160bb5b6d6974_5LNF2s3qPI.png?height=170&lazyload=true&maxWidth=600&width=1544)
