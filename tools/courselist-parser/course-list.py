'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-crawler](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 21, 2020

'''

import requests
import re
from bs4 import BeautifulSoup
from collections import namedtuple

Course = namedtuple('Course',
                    [
                        'id',  # 课程序号
                        'name',  # 课程名称
                        'code',  # 课程代码
                        'type',  # 课程类型
                        'credit',  # 学分
                        'teacher',  # 授课老师
                        'time_array',  # 上课时间
                        'place',  # 上课地点
                        'campus',  # 校区
                        'plan_count',  # 计划人数
                        'selected_count',  # 已选人数
                        'arranged_class',  # 配课班
                        'note',  # 备注
                    ])

CourseTime = namedtuple('CourseTime',
                        [
                            'week_index',  # tuple, 如 (1, 18) 表示 第 1..18 周
                            'type',  # 0 每周, 1 单周, 2 双周
                            'day_index',  # tuple, 周 1..7
                            'day_range',  # tuple, 第 1..12 节
                        ])

'''
    Parse range string, for example, '1-20' -> (1, 20), '3' -> (3, 3)
'''


def parse_range_string(range_string: str) -> (int, int):
    l = range_string.split('-')
    if len(l) == 2:
        return int(l[0]), int(l[1])
    else:
        return int(l[0]), int(l[0])


def parse_place_string(place_string: str) -> list:
    return place_string.split(',')


def parse_class_string(class_string: str) -> list:
    return class_string.split(', ')


'''
    Parse time string, return a list of CourseTime objects. For example:
    第4-11周,周1,第1-2节;  -> (4, 11), '', 1, (1, 2)
'''


def parse_time_string(time_string: str) -> list:
    time_array = time_string.split(';')
    result: list = []

    for each_time_arrangement in time_array:
        for item in re.findall(r'第(\d+(?:-\d+)?)周([*]{0,2}),周(\d),第(\d+(?:-\d+)?)节', each_time_arrangement):
            week_range = parse_range_string(item[0])
            class_range = parse_range_string(item[3])

            result.append(CourseTime(
                week_index=week_range,
                type=len(item[1]),
                day_index=int(item[2]),
                day_range=class_range
            ))
    return result


# Load course list page.
html = open("课程列表页面.html", encoding="GBK")
page = BeautifulSoup(html, "html5lib")

# Select each course line.
each_course_html = page.select("table tr[bgcolor=\"#efefef\"]")
course_list: list = []

for cols in each_course_html:
    # Convert each column to string and trim whites.
    cols = [x.text.strip() for x in cols.select('td')]
    # Convert to Course obejct.
    course = Course(
        id=cols[0],
        name=cols[1],
        code=cols[2],
        type=cols[3],
        credit=float(cols[4]),
        teacher=cols[5],
        time_array=parse_time_string(cols[6]),
        place=parse_place_string(cols[7]),
        campus=cols[8],
        plan_count=int(cols[9]),
        selected_count=int(cols[10]),
        arranged_class=parse_class_string(cols[12]),
        note=cols[13],
    )
    course_list.append(course)

for course in course_list:
    print(course)
