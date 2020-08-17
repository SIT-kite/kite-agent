# 上应小风筝数据收集模块

 ## 项目概要
  本项目旨在为上海应用技术大学的学生提供校园信息整合与管理服务。  
  数据收集模块与公网中核心节点保持连接，负责接收命令、数据爬取，并整合转发给公网核心节点。  
  后端代码见 [kite-server](https://github.com/sunnysab/kite-server) ，项目使用 [Rust](https://www.rust-lang.org/) 语言编写。

 ## 功能
 
 ### 通用
 
 - [x] 校园网登录
 
 ### 信息门户
 
 - [ ] 最新通知
 
 ### 教务系统
 
 - [x] 获取最新课程列表
 - [x] 获取个人教学计划
 - [x] 获取各专业教学计划
 - [ ] 选课
 - [ ] 退选课 
 
 ### 第二课堂系统
 
 - [x] 获取最近活动列表
 - [x] 获取并计算我的得分
 - [x] 获取我参加的活动
 - [ ] 申请活动
 - [x] 获取活动详情
 
 ### 校园卡业务
 
 - [x] 获取消费记录
 - [x] 电费查询
 
 ## 目标平台
 
 - Linux x86_64
 - Linux on Arm
 
 ## 运行
 
 请先确保系统中已预装有 rust 编程环境（rustc、cargo等），并已连接上互联网。
 ```bash
cargo run 
```

## 贡献者

- [sunnysab](https://github.com/sunnysab)
- [peanut996](https://github.com/peanut996)



## 开源协议

[GPL v3](https://github.com/sunnysab/kite-crawler/blob/master/LICENSE) © 上海应用技术大学易班

除此之外，您不能将本程序用于各类竞赛、毕业设计、论文等。