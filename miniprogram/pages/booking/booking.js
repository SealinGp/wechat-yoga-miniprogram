const utils = require('../../utils')
const shared = require('../../utils/shared')
const app = getApp();

Page({
  data: {
    app,
    // 距离今天的天数，用于切换本周下周
    offset: 0,
    // 用户选定单位为秒钟的时间
    selectedTime: 0,
    isPreviewing: false
  },
  async onLoad() {
    wx.showShareMenu({
      withShareTicket: true,
      menus: ['shareAppMessage', 'shareTimeline']
    })
    wx.setNavigationBarTitle({
      title: app.globalData.title
    })
    // 仅包含日期不包含小时等的单位为秒钟的今天的时间
    this.data.selectedTime = new Date().setHours(0, 0, 0, 0) / 1000;
    // 设置底部栏
    this.getTabBar().setData({
      items: [{
        name: "首页",
        src: "home",
        href: "index"
      }, {
        name: "团课",
        src: "big",
        href: "booking"
      }, {
        name: "小班",
        src: "small",
        href: "small"
      }, {
        name: "私教",
        src: "one",
        href: "one",
        page: true
      }],
      selected: 1
    })
    this.loadData()
  },
  navigate(e) {
    utils.navigate(e)
  },
  async loadData() {
    let openid = (await app.getOpenId()) || "";
    this.setData({
      holiday: false,
      loading: true
    })
    try {
      const response = await shared.request({
        url: `/yoga/lessons?start=${this.data.selectedTime}&openid=${openid}&class_type=4`,
        method: 'GET'
      });
      
      if (response && response.statusCode === 200) {
        const lessons = response.data;
        // Process lessons to add time and status information
        const processedLessons = this.processLessons(lessons, openid);
        this.setData({
          lessons: processedLessons,
          loading: false
        });
      } else {
        throw new Error('Failed to load lessons');
      }
    } catch (error) {
      console.error('Failed to load lessons:', error);
      this.setData({
        holiday: true,
        loading: false,
        lessons: []
      });
    }
  },

  // Process lessons to add time formatting and status
  processLessons(lessons, openid) {
    if (!Array.isArray(lessons)) return [];
    
    return lessons.map(lesson => {
      const now = Date.now() / 1000;
      const lessonTime = lesson.date_time + lesson.start_time;
      const endTime = lesson.date_time + lesson.end_time;
      
      // Add time formatting
      lesson.time = this.formatTime(lesson.start_time, lesson.end_time);
      lesson.date = this.formatDate(lesson.date_time);
      
      // Add status and mode
      if (now - endTime > 3600) {
        lesson.mode = 1;
        lesson.label = "已完成";
      } else if (now > lessonTime) {
        lesson.mode = 16;
        lesson.label = "正在上课";
      } else if (lessonTime - now < 3600) {
        lesson.mode = 8;
        lesson.label = "准备上课";
      } else {
        const hidden = lesson.hidden || 0;
        const peoples = lesson.peoples || 0;
        const users = lesson.users || [];
        const count = users.length;
        
        if (hidden === -1) {
          lesson.mode = 4;
          lesson.label = "已取消";
        } else {
          const userBooking = users.find(user => user.open_id === openid);
          if (userBooking) {
            lesson.mode = 64;
            lesson.label = "取消预约";
            lesson.reservation_id = userBooking.reservation_id;
          } else if (count >= peoples) {
            lesson.mode = 2;
            lesson.label = "已满额";
          } else {
            lesson.mode = 32;
            lesson.label = "预约";
          }
        }
      }
      
      return lesson;
    }).sort((a, b) => {
      const timeA = a.date_time + a.start_time;
      const timeB = b.date_time + b.start_time;
      return timeA - timeB;
    });
  },

  formatTime(startTime, endTime) {
    const formatSeconds = (seconds) => {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return `${hours}:${minutes.toString().padStart(2, '0')}`;
    };
    return `${formatSeconds(startTime)}-${formatSeconds(endTime)}`;
  },

  formatDate(dateTime) {
    const date = new Date(dateTime * 1000);
    const weekdays = ['日', '一', '二', '三', '四', '五', '六'];
    return `${date.getMonth() + 1}月${date.getDate()}日周${weekdays[date.getDay()]}`;
  },
  onShareAppMessage() {
    return {
      title: app.globalData.title
    };
  },
  onBookingBarSubmit(evt) {
    if (evt.detail === "1") {
      this.setData({
        offset: 0
      });

    } else {
      this.setData({
        offset: 7
      });
    }
  },
  onDailyScheduleSubmit(evt) {
    this.data.selectedTime = evt.detail
    this.loadData()
  },
  async onBookingItemSubmit(evt) {
    const item = evt.detail;
    if (item.mode & 6) {
      await this.unbook(item)
    } else if (((item.mode & 8) || (item.mode & 128))) {
      await this.book(item)
    }
  },
  onClick(e) {
    const { id, bookid, mode } = e.currentTarget.dataset;
    if (mode === 32) {
      this.book(id)
    } else if (mode === 64) {
      this.unbook(bookid)
    }
  },
  // 预约课程
  async book(id) {
    let result = await checkUserAvailability(app);
    if (!result) {
      wx.navigateTo({
        url: `/pages/login/login?return_url=${encodeURIComponent(`/pages/booking/booking`)}`
      })
      return;
    }
    try {
      let openid = (await app.getOpenId()) || "";
      const response = await shared.request({
        url: `/yoga/book?id=${id}&openid=${openid}`,
        method: 'GET'
      });
      
      if (response && response.statusCode === 200) {
        const result = parseFloat(response.data);
        if (result > 0) {
          await this.loadData();
          wx.showToast({
            title: '预约成功',
            icon: 'success'
          });
        } else {
          wx.showModal({
            title: '信息',
            content: '请您购买会员卡',
            success: res => {
              if (res.confirm) {
                wx.navigateTo({
                  url: '/pages/membership/membership'
                });
              }
            }
          })
        }
      }
    } catch (error) {
      console.error('预约失败:', error);
      wx.showToast({
        title: '预约失败',
        icon: 'error'
      });
    }
  },
  // 取消已预约的课程
  async unbook(bookid) {
    try {
      const openid = await app.getOpenId() || "";
      const response = await shared.request({
        url: `/yoga/unbook?id=${bookid}&openid=${openid}`,
        method: 'GET'
      });
      
      if (response && response.statusCode === 200) {
        await this.loadData();
        wx.showToast({
          title: '取消成功',
          icon: 'success'
        });
      }
    } catch (error) {
      console.error('取消预约失败:', error);
      wx.showToast({
        title: '取消失败',
        icon: 'error'
      });
    }
  },
  onPreview(e) {
    this.setData({
      previewText:e.target.dataset.name,
      previewImage:e.target.dataset.image,
      isPreviewing:true
    })
  },
  onClosePreview(){
    this.setData({
      isPreviewing:false
    })
  }
}

)
async function checkUserAvailability(app) {
  if (!app.globalData.openid) {
    return false;
  }
  if (app.globalData.userId) {
    return true;
  }
  let result;
  try {
    const response = await shared.request({
      url: `/yoga/user/query?openid=${app.globalData.openid}`,
      method: 'GET'
    });
    
    if (response && response.statusCode === 200) {
      result = response.data;
      //TODO: check
      if (!result || !result.nick_name) {
        return false;
      }
      app.globalData.userId = result;
      return true;
    }
    return false;
  } catch (error) {
    console.error(error);
    return false;
  }
}