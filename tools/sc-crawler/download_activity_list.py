'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-crawler](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 14, 2020

'''
import requests
import re
from bs4 import BeautifulSoup

# Activity list.
LIST_URL = r'http://sc.sit.edu.cn/public/activity/activityList.action?pageNo={}&pageSize={}&categoryId=&activityName='
COOKIES = 'xxxx'


# Get activities from a single activity list page.
# Return value is a list of tuple with (activity_id, title, start_time).
def resolveList(text: str) -> list:
    result = []
    html_page = BeautifulSoup(text, 'html5lib')
    items = html_page.select('.ul_7 li')

    for item in items:
        link = item.find('a')
        title = link.text.replace('·\n', '')
        activity_id = re.search('\d+', link['href'])
        start_time = item.find_all('span')[0].text

        result.append((int(activity_id.group()), title, start_time))
    return result


# Request target activity list page.
def getListPage(page_index: int) -> str:
    response = requests.get(LIST_URL.format(page_index, 200), headers={
        "Cookie": COOKIES
    }, timeout=5)
    # Return None if the request failed.
    return response.text


def getActivityList(page_index: int) -> list:
    text = getListPage(page_index)
    if text is None:
        print("Could not get activity list: No response.")
        return None
    return resolveList(text)


if __name__ == '__main__':
    output_file = open('activity_list.txt', 'w+', encoding='utf-8')

    for i in range(1, 81):
        for item in getActivityList(i):
            output_file.write('{}, {}, {}\n'.format(item[0], item[1], item[2]))
        output_file.flush()

    output_file.close()
