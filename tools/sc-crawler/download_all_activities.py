'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 14, 2020

'''
import requests
import re
import time
import download_activity_list
from bs4 import BeautifulSoup
from collections import namedtuple

# Activity detail
ACITIVITY_URL = 'http://sc.sit.edu.cn/public/activity/activityDetail.action?activityId={}'
COOKIES = 'xxxx'

# Convert to headers dict for performance.
headers = {
    'Cookie': COOKIES
}

# Activity Strutcure
Activity = namedtuple('Activity',
                      ['id',  # 活动 ID
                       'title',  # 活动标题
                       'start_time',  # 开始日期和时间
                       'sign_time',  # 开始签到日期和时间
                       'end_time',  # 结束日期和时间
                       'place',  # 地点
                       'duration',  # 时长
                       'manager',  # 负责人
                       'contact',  # 负责人联系方式
                       'organizer',  # 主办方
                       'undertaker',  # 承办方
                       'description',  # 活动描述
                       'attachment'  # 附件列表: (url, name)
                       ]
                      )


# Get activity detail page
def getActivityPage(activity_id) -> str:
    url = ACITIVITY_URL.format(activity_id)

    response = requests.get(url, headers=headers, timeout=5)
    return response.text


# Resolve detail page.
def getDetails(text: str) -> Activity:
    html_page = BeautifulSoup(text, 'html5lib')
    # Get title
    # \xa0 is the whitespace in unicode, which beautifulsoup decode '&nbsp;' to.
    title = html_page.find('h1').text.replace('\xa0', '')

    # Get banner and prarmeters.
    banner = html_page.select_one('div[style=" color:#7a7a7a; text-align:center"]').text
    banner = banner.replace('\xa0', '')
    id = re.search(r'活动编号：(\d{7})', banner).group(1)
    start_time = re.search(r'活动开始时间：(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+)', banner).group(1)
    place = re.search(r'活动地点：(.*)', banner).group(1)
    duration = re.search(r'活动时长：(.*) 分钟', banner).group(1)
    manager = re.search(r'负责人：(.*)', banner).group(1)
    contact = re.search(r'负责人电话：(.*)', banner).group(1)
    organizer = re.search(r'主办方：(.*)', banner).group(1)
    undertaker = re.search(r'承办方：(.*)', banner).group(1)
    start_end_time = re.search(r'刷卡时间段：(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+).*--至--.*(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+)',
                               banner)
    sign_time = start_end_time.group(1)
    end_time = start_end_time.group(2)

    # Get description
    description_secton = html_page.select_one('div[style="padding:30px 50px; font-size:14px;"]');
    description = description_secton.get_text()
    description = description.strip()
    description = re.sub('[\n\t\xa0 ]+', '<br>', description)

    # Get attachments
    attachment_list = []
    for each_attachment in description_secton.find_all('a'):
        try:
            url = each_attachment['href']
            file = each_attachment.text
            attachment_list.append((id, url, file))
        except:
            continue

    return Activity(id=id, title=title, sign_time=sign_time, start_time=start_time, end_time=end_time,
                    duration=duration,
                    place=place, description=description, contact=contact, manager=manager, organizer=organizer,
                    undertaker=undertaker,
                    attachment=attachment_list)


if __name__ == "__main__":
    activity_list = []
    attachment_list = []

    fp = open('activities.txt', 'a+', encoding='utf-8')
    attachments_file = open('attachments.txt', 'a+', encoding='utf-8')
    error_file = open('error.txt', 'a+', encoding='utf-8')

    # Get activity list.
    # for i in range(1, 81):
    #    activity_list.extend(download_activity_list.getActivityList(i))
    activity_list = open('error.txt', 'r', encoding='utf-8').readlines()
    activity_id_list = [int(x.split(',')[0]) for x in activity_list]

    # Skip.
    i = 0
    for activity_id in activity_id_list:
        i += 1
        if i % 300 == 0:
            print("Current: ", i)
            time.sleep(5)

        # Get detail page.
        try:
            page = getActivityPage(activity_id)
        except:
            error_file.write("{}\n".format(activity_id))
            time.sleep(5)
            continue

        try:
            activity = getDetails(page)
            fp.write("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n".format(
                activity.id, activity.title, activity.start_time, activity.sign_time, activity.end_time, activity.place,
                activity.duration, activity.manager, activity.contact, activity.organizer, activity.undertaker,
                activity.description))
            if len(activity.attachment) != 0:
                attachment_list.extend(activity.attachment)

        except:
            print("Error resolving activity ", activity_id)

    for attachment in attachment_list:
        attachments_file.write("{}, {}, {}\n".format(attachment[0], attachment[1], attachment[2]))

    fp.close()
    attachments_file.close()
    error_file.close()
