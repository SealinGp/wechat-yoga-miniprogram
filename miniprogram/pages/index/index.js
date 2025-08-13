const app = getApp();
const shared = require("../../utils/shared");

Page({
  data: {
    app,
    enabled: false
  },
  // 该页面加载时运行一次的方法
  async onLoad() {
    this.initializeTopBar();
    this.loadIndexData();
    // 启用分享小程序的功能
    wx.showShareMenu({
      withShareTicket: true,
      menus: ['shareAppMessage', 'shareTimeline']
    })
    // 设置顶部导航栏的标题
    wx.setNavigationBarTitle({
      title: app.globalData.title
    });
    // 设置底部工具栏
    this.getTabBar().setData({
      items: [{
        name: "首页",
        // 图标的路径
        src: "home",
        // 导航的页面
        href: "index"
      }, {
        name: "约课",
        src: "book",
        href: "booking"
      }, {
        name: "已约",
        src: "booked",
        href: "booked"
      }, {
        name: "我的",
        src: "user",
        href: "user"
      }

      ],
      selected: 0
    });
  },

  // 加载首页数据
  async loadIndexData() {
    try {
      const response = await shared.request({
        url: `/yoga/index`,
        method: 'GET'
      });
      
      if (response && response.statusCode === 200) {
        const data = response.data;
        // 处理notices的时间显示
        if (data.notices && Array.isArray(data.notices)) {
          data.notices.forEach(notice => {
            if (notice.updated_time) {
              const now = Date.now() / 1000;
              const dif = now - notice.updated_time;
              if (dif > 31536036) {
                notice.timeago = `${Math.floor(dif / 31536036)}年之前`;
              } else if (dif > 2628003) {
                notice.timeago = `${Math.floor(dif / 2628003)}月之前`;
              } else if (dif > 86400) {
                notice.timeago = `${Math.floor(dif / 86400)}天之前`;
              } else if (dif > 3600) {
                notice.timeago = `${Math.floor(dif / 3600)}小时之前`;
              } else {
                notice.timeago = "刚刚";
              }
            }
          });
        }
        
        this.setData({
          ...data,
          enabled: true
        });
      }
    } catch (error) {
      console.error('加载首页数据失败:', error);
      this.setData({
        enabled: true
      });
    }
  },
  navigate(e) {
    shared.navigate(e);
  },
  // 设置分享时的标题
  onShareAppMessage() {
    return {
      title: app.globalData.title
    };
  },
  onHomeActionsSubmit(evt) {
      
    if (evt.detail === 2) {
      wx.switchTab({
        url: `/pages/booking/booking`
      })
    } else if (evt.detail === 3) {
      wx.navigateTo({
        url: `/pages/one/one`
      })
    }else if (evt.detail === 5) {
      wx.navigateTo({
        url: `/pages/sudoku/sudoku`
      })
    }else if (evt.detail === 6) {
      wx.navigateTo({
        url: `/pages/market/market`
      })
    } else if (evt.detail === 7) {
      wx.navigateTo({
        url: `/pages/notices/notices`
      })
    }
  },
  // 导航到公告页面
  onHomeNoticeSubmit(evt) {

    wx.navigateTo({
      url: `/pages/notice/notice?id=${evt.detail}`
    })
  },
  // 导航到老师页面
  onTeacherSubmit(evt) {
    wx.navigateTo({
      url: `/pages/teacher/teacher?id=${evt.detail}`
    })
  },
  // 导航到预约页面
  onHomeBookedSubmit(evt) {
    wx.switchTab({
      url: `/pages/booking/booking`
    })
  },

  // 初始化顶部导航栏
  initializeTopBar() {
    const { navigationHeight, navigationTop, paddingLeft } = shared.getNavigationBarSize();
    this.setData({
      navigationHeight,
      navigationTop,
      navigationPaddingLeft: paddingLeft,
      navigationTitleFontSize: navigationHeight / 6 * 2,
      navigationSubTitleFontSize: navigationHeight / 6 * 1.5,
      navigationGap: navigationHeight / 6 * .3
    });

    // 设置时间和天气信息
    this.setupTimeAndWeather();
  },

  // 设置时间和天气
  setupTimeAndWeather() {
    const weather = "长沙市 晴 25° 微风2级"; // 默认天气信息
    const date = this.getLunarDate(); // 获取农历日期
    
    this.setData({
      weather: weather,
      date: date,
    });
    
    // 设置时间显示
    this.updateTime();
    this.data.timer = setInterval(() => {
      this.updateTime();
    }, 1000);
  },

  // 更新时间显示
  updateTime() {
    const now = new Date();
    const timeString = `北京时间 ${now.getHours()}点${now.getMinutes()}分${now.getSeconds()}秒`;
    this.setData({
      bj: timeString
    });
  },

  // 获取农历日期（简化版本）
  getLunarDate() {
    const now = new Date();
    const month = now.getMonth() + 1;
    const day = now.getDate();
    return `农历 ${month}月${day}日`; // 简化版本，实际应该转换为真正的农历
  },

  onUnload() {
    // 页面卸载时清除定时器
    if (this.data.timer) {
      clearInterval(this.data.timer);
    }
  }
})