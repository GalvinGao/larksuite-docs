---
document_id: '7233232281317720070'
directory_id: '7199928167142260741'
title: 步骤三：下载并运行示例代码
full_path: /home/todos-daily-reminder-of-weekly-report/download-and-run-the-sample-code
breadcrumb:
- Home
- Todos daily reminder of weekly report in Wiki
- 'Step 3: Download and run the sample code'
document_type: GuideDocumentType
updated_at: 2023-08-11T09:44:25Z
source_url: https://open.larksuite.com/document/home/todos-daily-reminder-of-weekly-report/download-and-run-the-sample-code
---

# 步骤三：下载并运行示例代码

在本步骤，你将下载并运行教程提供的示例代码。示例代码使用 Python 语言编写，请确保你已安装 Python 运行环境。

## 步骤一：下载示例代码
1. 执行以下命令，下载 [示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e137480cc912a336220abc59e7ee194_dlBrtvQxo2.zip) 到本地。

   * Mac OS 或 Linux执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e137480cc912a336220abc59e7ee194_dlBrtvQxo2.zip -o todo_reminder.zip
      unzip todo_reminder.zip
      cd todo_reminder/python
      ```

   * Windows执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e137480cc912a336220abc59e7ee194_dlBrtvQxo2.zip -o todo_reminder.zip
      todo_reminder.zip
      cd todo_reminder/python
      ```
:::html
<md-td>
2. 执行以下命令，修改`.env`文件中的凭证信息。
   ```PowerShell
   vim .env
   ```

     :::note
     Windows用户可直接使用记事本进行修改。
     :::

    1. 按 `i` 进入编辑模式，然后修改 `.env` 文件中的凭证信息。
        * **APP_ID** 和 **APP_SECRET** 应用凭证信息可以在[开发者后台](https://open.larksuite.com/app)的 **凭证与基础信息** 页查看。

        	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/be069cb96e07a3512076c0e6aba814db.png" style="width:70%"/>
        * **WIKI_SPACE_ID** 可以在知识库 **设置** 页面获取。

        	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a84e12d80292ad0bb5533b0eecb93941.png" style="width:70%"/>
        * **WIKI_NODETOKEN** 可以在文档页面获取。

        	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/acd82f0b8eb9cc14451ac637c24695f2.png" style="width:70%"/>

      2. 修改完成后，按 `:wq` 保存退出。
          :::note
          私有化部署时要修改`.env`文件中 **Lark_HOST** 为私有化部署所用的域名。
          :::
      3. 打开`reminder.cron`修改cron表达式的参数。
      
  			以下cron表达式表示：每周一和周五上午九点进入   `/home/app`   目录，运行   reminder.py   文件。
          ```Python
          # Notice at every weekday at 9:00
          0 9 * * 1-5 cd /home/app && /usr/local/bin/python reminder.py >> reminder.log 2>&1
          ```
</md-td>
:::

## 步骤二：运行示例代码
你可以参考以下两种方式运行示例代码。

### Docker运行
如果您已有[Docker](https://www.docker.com/)环境，那么直接执行以下命令即可。
* Mac OS或Linux执行以下命令：
   ```Shell
   sh exec.sh
   ```

* Windows执行以下命令：
   ```Shell
   .\exec.ps1
   ```


### 本地运行
1. 执行以下命令，创建并激活Python虚拟环境。
   * Mac OS 或 Linux执行以下命令。
      ```Shell
      python3 -m venv venv
      . venv/bin/activate
      ```

   * Windows执行以下命令。
      ```Shell
      python3 -m venv venv
      venv\Scripts\activate
      ```

2. 执行以下命令，安装代码依赖。
   ```Shell
   pip3 install -r requirements.txt
   ```

3. 依赖安装成功后，执行以下命令运行示例代码。
   ```Shell
   python3 reminder.py
   ```



