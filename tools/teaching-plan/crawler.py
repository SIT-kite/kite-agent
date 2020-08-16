'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 28, 2020

'''

import requests
from pc_parser import PlannedCourse, getCourseList

URL = 'http://ems.sit.edu.cn:85/student/course.jsp'
COOKIES = 'xxx'

session = requests.session()


def queryCourseList(major_id: str, year: str, course_sort: int) -> list:
    r = session.get(URL, params={
        'majorId': major_id,
        'enterYear': year,
        'courseBigSortId': course_sort,
    }, 
    headers={
        'Cookie': COOKIES,
    })

    if r.status_code == 200:
        return getCourseList(r.text)
    print('r.status_code = ', r.status_code)
    return []


if __name__ == '__main__':
    with open('majors.csv', 'r', encoding='utf-8') as f:
        content = f.readlines()
    majors = [x.split(',')[1] for x in content]
    majors = [x for x in majors if x != '']

    with open('course_plan.csv', 'w+', encoding='utf-8') as f:
        for major in majors:
            #for year in range(2015, 2021):
            for year in range(2017, 2020):
                for course_sort in range(1, 8):
                    courses = queryCourseList(major, year, course_sort)
                    for c in courses:
                        f.write('{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n'.format(
                            major, year, course_sort, c.code, c.name, c.has_test, c.credit, c.theory_hour, c.practice_hour, 
                            c.department, c.term))
                f.flush()
