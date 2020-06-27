


| 编号 | 文件名                | 文件描述      | 方式                                                       | 链接                                                         |
| ---- | --------------------- | ------------- | ------------------------------------------------------------ | ---- |
| 1    | 第二课堂最近活动.html | 最近200个活动 | GET | [Link](http://sc.sit.edu.cn/public/activity/activityList.action?pageNo=1&pageSize=200&categoryId=&activityName=) |
| 2    | 第二课堂得分页面.html | 页面有pageSize，pageNo 参数  |GET |[Link](http://sc.sit.edu.cn/public/pcenter/scoreDetail.action?pageSize=200) |
| 3 | 电费查询页面.html | 页面参数 `fjh` 用于指定房间号 | POST | [Link](http://card.sit.edu.cn/dk_xxmh.jsp?actionType=init&selectstate=on&fjh=103110) |
| 4 | 消费记录页面.html | 消费记录，已脱敏。注意，学校会定时清消费记录 | GET | [Link](http://card.sit.edu.cn/personalxiaofei.jsp?page=1&from=20200101&to=20200431) |
| 5 | 成绩查询页面.html | 成绩页面，已脱敏。注意，HTML中有错误 | POST | [Link](http://ems.sit.edu.cn:85/student/graduate/scorelist.jsp?yearterm=2020%B4%BA&studentID=学号) |
| 6 | 个人教学计划完成情况.html | 无需评教查看成绩 | GET | [Link](http://ems.sit.edu.cn:85/student/graduate/viewcreditdetail.jsp?) |
| 7 | 课程列表页面.html | 课程列表。注意，部分字符串格式不正确，可能需要人工处理 | POST | [Link](http://ems.sit.edu.cn:85/student/selCourse/mycourselist.jsp)<br>yearTerm=2018春<br>&kcxu=<br>&kcdm=<br>&kcmc= |
| 8 | 我的课表页面.html | 课表页面。直接从“课程详细信息”中提取课程列表、然后查询该课程时间即可，不要直接解析上方课表。 | POST | [Link](http://ems.sit.edu.cn:85/student/selCourse/syllabuslist.jsp)<br>yearTerm=2019春<br>&cType=2（2为实践课）<br>&yearTerm2=2019-2020%B5%DA1%D1%A7%C6%DA |
| 9 | 教学计划查询页面 | 教学计划查询页面，注意，HTML 页面中有大量错误。课程类别范围为1~7，在HTML中有规定，可以硬编码。 | GET | [Link](http://ems.sit.edu.cn:85/student/course.jsp)<br>majorId=B110101<br>&enterYear=2019<br>&courseBigSortId=1（课程类别） |

**注意**

1. 尽量在解析时选择 GBK 编码，以和网页保持一致。
2. 选择合适的解析器和解析方法，以匹配为主，避免因遇到 HTML 格式错误而导致解析失败。